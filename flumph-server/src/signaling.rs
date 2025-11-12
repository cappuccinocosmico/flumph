
// src/signaling.rs

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post, Router},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::webrtc_server::WebRTCServer;

#[derive(Deserialize)]
pub struct OfferRequest {
    pub camera_id: String,
    pub offer: String,
}

#[derive(Serialize)]
pub struct OfferResponse {
    pub answer: String,
}

// Handles a WebRTC offer from a camera.
pub async fn handle_offer(
    axum::extract::Extension(webrtc_server): axum::extract::Extension<Arc<WebRTCServer>>,
    Json(payload): Json<OfferRequest>,
) -> impl IntoResponse {
    match webrtc_server
        .handle_offer(payload.camera_id, payload.offer)
        .await
    {
        Ok(answer) => (StatusCode::OK, Json(OfferResponse { answer })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

// Creates a new Axum router for the signaling server.
pub fn create_router(webrtc_server: Arc<WebRTCServer>) -> Router {
    Router::new()
        .route("/api/signaling/offer", post(handle_offer))
        .layer(axum::extract::Extension(webrtc_server))
}
