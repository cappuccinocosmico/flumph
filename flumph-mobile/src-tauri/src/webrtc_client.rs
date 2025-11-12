//! WebRTC Client implementation

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_gatherer_state::RTCIceGathererState;
use webrtc::ice_transport::ice_gathering_state::RTCIceGatheringState;
use webrtc::interceptor::registry::Registry;

use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_AV1};
use webrtc::api::APIBuilder;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::ice_transport::ice_candidate::RTCIceCandidate;
use webrtc::ice_transport::ice_connection_state::RTCIceConnectionState;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::{TrackLocal, TrackLocalWriter};

/// WebRTC Client manages peer connection and data channels
pub struct WebRTCClient {
    peer_connection: Arc<RTCPeerConnection>,
    video_track: Arc<TrackLocalStaticRTP>,
    // control_channel: Arc<RTCDataChannel>,
}

impl WebRTCClient {
    pub fn peer_connection(&self) -> Arc<RTCPeerConnection> {
        Arc::clone(&self.peer_connection)
    }

    pub fn video_track(&self) -> Arc<TrackLocalStaticRTP> {
        Arc::clone(&self.video_track)
    }
}

impl WebRTCClient {
    pub async fn new() -> Result<Self> {
        // Create a MediaEngine object to configure the codecs
        let mut m = MediaEngine::default();

        m.register_codec(
            webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecParameters {
                capability: RTCRtpCodecCapability {
                    mime_type: MIME_TYPE_AV1.to_owned(),
                    clock_rate: 90000,
                    channels: 0,
                    sdp_fmtp_line: "".to_owned(),
                    rtcp_feedback: vec![],
                },
                payload_type: 96, // Example payload type
                ..Default::default()
            },
            webrtc::rtp_transceiver::rtp_codec::RTPCodecType::Video,
        )?;

        // Create a InterceptorRegistry. This is the user configurable RTP/RTCP Pipeline.
        // This provides better control in the off chance you'd like to modify the RTP
        // or RTCP packets before they're sent/received.
        let mut registry = Registry::new();

        // Use the default set of Interceptors
        registry = register_default_interceptors(registry, &mut m)?;

        // Create the API object with the MediaEngine and InterceptorRegistry
        let api = APIBuilder::new()
            .with_media_engine(m)
            .with_interceptor_registry(registry)
            .build();

        // Prepare the configuration
        let config = RTCConfiguration {
            ice_servers: vec![], // TODO: Add STUN/TURN servers
            ..Default::default()
        };

        // Create a new PeerConnection
        let peer_connection = Arc::new(api.new_peer_connection(config).await?);

        // Create a video track
        let video_track = Arc::new(TrackLocalStaticRTP::new(
            RTCRtpCodecCapability {
                mime_type: MIME_TYPE_AV1.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: "".to_owned(),
                rtcp_feedback: vec![],
            },
            "video".to_owned(),
            "webrtc-video".to_owned(),
        ));

        // Add this newly created track to the PeerConnection
        let rtp_sender = peer_connection
            .add_track(Arc::clone(&video_track) as Arc<dyn TrackLocal + Send + Sync>)
            .await?;

        // Read incoming RTCP packets. Block until we manage to send them.
        tokio::spawn(async move {
            let mut rtcp_buf = vec![0u8; 1500];
            while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
            Result::<()>::Ok(())
        });

        // Set the handler for ICE connection state
        // This will notify you when the peer connection's ICE connection state changes.
        peer_connection.on_ice_connection_state_change(Box::new(
            move |p: RTCIceConnectionState| {
                Box::pin(async move {
                    info!("ICE Connection State has changed to {}", p);
                    if p == RTCIceConnectionState::Connected {
                        info!("WebRTC connection established.");
                    }
                })
            },
        ));

        // Set the handler for PeerConnection state
        // This will notify you when the peer connection's state changes.
        peer_connection.on_peer_connection_state_change(Box::new(
            move |s: RTCPeerConnectionState| {
                Box::pin(async move {
                    info!("Peer Connection State has changed to {}", s);
                    if s == RTCPeerConnectionState::Connected {
                        info!("Peer connection is connected.");
                    } else if s == RTCPeerConnectionState::Failed
                        || s == RTCPeerConnectionState::Closed
                    {
                        error!("Peer connection failed or closed.");
                    }
                })
            },
        ));

        // Register data channel creation handling
        peer_connection.on_data_channel(Box::new(move |dc| {
            let dc_label = dc.label().to_owned();
            let dc_label_2 = dc_label.clone();
            let dc_id = dc.id();
            info!("New DataChannel {} {}", dc_label, dc_id);

            Box::pin(async move {
                dc.on_message(Box::new(move |msg: DataChannelMessage| {
                    let dc_label_n = dc_label_2.clone();
                    Box::pin(async move {
                        let msg_str = String::from_utf8_lossy(&msg.data);
                        info!(
                            "Data channel '{}'-'{}' received message: {}",
                            &dc_label_n, dc_id, msg_str
                        );
                        // TODO: Process control commands
                    })
                }));
                dc.on_open(Box::new(move || {
                    Box::pin(async move {
                        info!("Data channel '{}'-'{}' open.", dc_label, dc_id);
                    })
                }));
            })
        }));

        Ok(Self {
            peer_connection,
            video_track,
        })
    }

    pub async fn create_offer(&self) -> Result<RTCSessionDescription> {
        let offer = self.peer_connection.create_offer(None).await?;

        // Create a channel that is blocked until ICE Gathering is complete
        let gather_complete = Arc::new(tokio::sync::Notify::new());
        let gather_complete_2 = Arc::clone(&gather_complete);
        self.peer_connection
            .on_ice_gathering_state_change(Box::new(move |s| {
                debug!("ICE Gathering State has changed to {}", s);
                if s == RTCIceGathererState::Complete {
                    gather_complete_2.notify_one();
                }
                Box::pin(async {})
            }));

        // Sets the LocalDescription, and starts ICE gathering
        self.peer_connection.set_local_description(offer).await?;

        // Block until ICE Gathering is complete, disabling this allows you to receive candidates
        // from the remote peer via the OnICECandidate handler
        gather_complete.notified().await;

        if let Some(local_desc) = self.peer_connection.local_description().await {
            Ok(local_desc)
        } else {
            Err(anyhow!(
                "Failed to get local description after ICE gathering"
            ))
        }
    }

    pub async fn set_remote_answer(&self, answer: RTCSessionDescription) -> Result<()> {
        self.peer_connection.set_remote_description(answer).await?;
        Ok(())
    }

    // pub async fn write_video_frame(&self, payload: &[u8]) -> Result<()> {
    //     self.video_track.write_rtp(payload).await?;
    //     Ok(())
    // }

    pub async fn close(&self) -> Result<()> {
        self.peer_connection.close().await?;
        Ok(())
    }
}
