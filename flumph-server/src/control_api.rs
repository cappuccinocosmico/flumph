
// src/control_api.rs

use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::webrtc_server::WebRTCServer;

#[derive(Serialize)]
pub struct CameraStatus {
    pub camera_id: String,
    pub is_streaming: bool,
}

#[derive(Deserialize)]
pub struct CameraConfig {
    pub bitrate: u32,
}

// Starts a camera stream.
pub async fn start_camera(
    Path(camera_id): Path<String>,
    axum::extract::Extension(webrtc_server): axum::extract::Extension<Arc<WebRTCServer>>,
) -> impl IntoResponse {
    webrtc_server.start_stream(&camera_id).await;
    (StatusCode::OK)
}

// Stops a camera stream.
pub async fn stop_camera(
    Path(camera_id): Path<String>,
    axum::extract::Extension(webrtc_server): axum::extract::Extension<Arc<WebRTCServer>>,
) -> impl IntoResponse {
    webrtc_server.stop_stream(&camera_id).await;
    (StatusCode::OK)
}

// Gets the status of a camera.
pub async fn get_camera_status(
    Path(camera_id): Path<String>,
    axum::extract::Extension(webrtc_server): axum::extract::Extension<Arc<WebRTCServer>>,
) -> impl IntoResponse {
    let is_streaming = webrtc_server.is_streaming(&camera_id).await;
    Json(CameraStatus { camera_id, is_streaming })
}

// Updates the configuration of a camera.
pub async fn update_camera_config(
    axum::extract::Extension(webrtc_server): axum::extract::Extension<Arc<WebRTCServer>>,
    Path(camera_id): Path<String>,
    Json(config): Json<CameraConfig>,
) -> impl IntoResponse {
    webrtc_server
        .update_config(&camera_id, config.bitrate)
        .await;
    (StatusCode::OK)
}

// Creates a new Axum router for the control API.
pub fn create_router(webrtc_server: Arc<WebRTCServer>) -> Router {
    Router::new()
        .route("/api/cameras/:id/start", post(start_camera))
        .route("/api/cameras/:id/stop", post(stop_camera))
        .route("/api/cameras/:id/status", get(get_camera_status))
        .route("/api/cameras/:id/config", post(update_camera_config))
        .layer(axum::extract::Extension(webrtc_server))
}
