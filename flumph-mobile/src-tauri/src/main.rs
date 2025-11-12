use tauri::{Manager, async_runtime::Mutex};
use tracing::{info, error};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use std::sync::Arc;

mod camera;
mod encoder;
mod webrtc_client;
mod config;

use camera::CameraManager;
use encoder::Av1Encoder;
use webrtc_client::WebRTCClient;
use config::AppConfig;

// State for Tauri commands
pub struct AppState {
    camera_manager: Mutex<Option<CameraManager>>,
    av1_encoder: Mutex<Option<Av1Encoder>>,
    webrtc_client: Mutex<Option<WebRTCClient>>,
    app_config: Mutex<AppConfig>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            camera_manager: Mutex::new(None),
            av1_encoder: Mutex::new(None),
            webrtc_client: Mutex::new(None),
            app_config: Mutex::new(AppConfig::default()),
        }
    }
}

#[tauri::command]
async fn get_config(state: tauri::State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.app_config.lock().await;
    Ok(config.clone())
}

#[tauri::command]
async fn save_config(state: tauri::State<'_, AppState>, new_config: AppConfig) -> Result<(), String> {
    let mut config = state.app_config.lock().await;
    *config = new_config;
    // TODO: Save to file
    Ok(())
}

#[tauri::command]
async fn start_stream(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("Attempting to start stream...");
    let mut app_config = state.app_config.lock().await;
    let config = app_config.clone();

    let mut camera_manager = state.camera_manager.lock().await;
    let mut av1_encoder = state.av1_encoder.lock().await;
    let mut webrtc_client = state.webrtc_client.lock().await;

    if camera_manager.is_none() {
        let resolution = nokhwa::Resolution::new(config.camera_resolution_width, config.camera_resolution_height);
        *camera_manager = Some(CameraManager::new(config.camera_index, resolution, config.camera_framerate).map_err(|e| e.to_string())?);
        camera_manager.as_mut().unwrap().start_stream().map_err(|e| e.to_string())?;
    }

    if av1_encoder.is_none() {
        *av1_encoder = Some(Av1Encoder::new(config.camera_resolution_width as usize, config.camera_resolution_height as usize, config.camera_framerate, config.encoder_bitrate_kbps).map_err(|e| e.to_string())?);
    }

    if webrtc_client.is_none() {
        *webrtc_client = Some(WebRTCClient::new().await.map_err(|e| e.to_string())?);
    }

    let pc = Arc::clone(webrtc_client.as_ref().unwrap().peer_connection());
    let video_track = Arc::clone(webrtc_client.as_ref().unwrap().video_track());

    // Create an offer
    let offer = webrtc_client.as_mut().unwrap().create_offer().await.map_err(|e| e.to_string())?;

    // TODO: Send offer to signaling server and receive answer
    info!("Generated WebRTC Offer: {}", offer.sdp);

    let camera_manager_arc = Arc::new(Mutex::new(camera_manager.take().unwrap()));
    let av1_encoder_arc = Arc::new(Mutex::new(av1_encoder.take().unwrap()));

    // Video capture and encoding loop
    tokio::spawn(async move {
        let mut camera_manager = camera_manager_arc.lock().await;
        let mut av1_encoder = av1_encoder_arc.lock().await;
        loop {
            match camera_manager.capture_frame().await {
                Ok(frame) => {
                    match av1_encoder.encode_frame(&frame) {
                        Ok(encoded_data) => {
                            if let Err(e) = video_track.write_rtp(&encoded_data).await {
                                error!("Failed to write RTP packet: {}", e);
                            }
                        },
                        Err(e) => error!("Failed to encode frame: {}", e),
                    }
                },
                Err(e) => error!("Failed to capture frame: {}", e),
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000 / config.camera_framerate as u64)).await;
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_stream(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("Attempting to stop stream...");
    let mut camera_manager = state.camera_manager.lock().await;
    if let Some(cam) = camera_manager.as_mut() {
        cam.stop_stream().map_err(|e| e.to_string())?;
    }
    let mut webrtc_client = state.webrtc_client.lock().await;
    if let Some(client) = webrtc_client.take() {
        client.close().await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    info!("Starting flumph-mobile application...");

    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![get_config, save_config, start_stream, stop_stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}