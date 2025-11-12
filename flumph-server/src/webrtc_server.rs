use crate::connection_manager::ConnectionManager;
use axum::{
    extract::Json,
    routing::post,
    Router,
};
use serde::{
    Deserialize,
    Serialize,
};
use webrtc::{
    api::{
        media_engine::MediaEngine,
        APIBuilder,
    },
    peer_connection::configuration::RTCConfiguration,
    peer_connection::sdp::session_description::RTCSessionDescription,
};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;

use webrtc::peer_connection::RTCPeerConnection;

// Represents a WebRTC server that manages peer connections.
pub struct WebRTCServer {
    // A map of camera IDs to their corresponding peer connections.
    connections: Arc<RwLock<HashMap<String, Arc<RTCPeerConnection>>>>,
    connection_manager: Arc<ConnectionManager>,
}

impl WebRTCServer {
    // Creates a new WebRTCServer.
    pub fn new(connection_manager: Arc<ConnectionManager>) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            connection_manager,
        }
    }

    // Handles a WebRTC offer from a camera.
    pub async fn handle_offer(
        &self,
        camera_id: String,
        offer: String,
    ) -> Result<String, anyhow::Error> {
        let mut m = webrtc::api::media_engine::MediaEngine::default();
        m.register_default_codecs()?;
        let mut registry = webrtc::interceptor::registry::Registry::new();
        registry = webrtc::api::interceptor_registry::register_default_interceptors(
            registry, &mut m,
        )?;
        let api = webrtc::api::APIBuilder::new()
            .with_media_engine(m)
            .with_interceptor_registry(registry)
            .build();

        let config = webrtc::peer_connection::configuration::RTCConfiguration::default();
        let peer_connection = Arc::new(api.new_peer_connection(config).await?);

        let connections = self.connections.clone();
        let camera_id_clone = camera_id.clone();
        peer_connection.on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
            let connections = connections.clone();
            let camera_id_clone = camera_id_clone.clone();
            if s == RTCPeerConnectionState::Failed {
                tokio::spawn(async move {
                    let mut conns = connections.write().await;
                    conns.remove(&camera_id_clone);
                });
            }
            Box::pin(async {})
        }));

        let offer = RTCSessionDescription::offer(offer)?;
        peer_connection.set_remote_description(offer).await?;
        let answer = peer_connection.create_answer(None).await?;
        peer_connection.set_local_description(answer.clone()).await?;

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        self.connection_manager.add_connection(camera_id.clone(), tx);

        let mut conns = self.connections.write().await;
        conns.insert(camera_id.clone(), peer_connection);

        Ok(answer.sdp)
    }

    // Starts a stream for a given camera ID.
    pub async fn start_stream(&self, camera_id: &str) {
        // Implementation for starting a stream
    }

    // Stops a stream for a given camera ID.
    pub async fn stop_stream(&self, camera_id: &str) {
        // Implementation for stopping a stream
    }

    // Checks if a camera is currently streaming.
    pub async fn is_streaming(&self, camera_id: &str) -> bool {
        self.connections.read().await.contains_key(camera_id)
    }

    // Updates the configuration for a given camera ID.
    pub async fn update_config(&self, camera_id: &str, bitrate: u32) {
        // Implementation for updating configuration
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    pub sdp: String,
}