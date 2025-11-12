// src/stream_relay.rs

use crate::connection_manager::ConnectionManager;
use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use bytes::Bytes;
use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    RwLock,
    broadcast::{self, Sender},
};

// Represents a stream relay that forwards video data to web viewers.
pub struct StreamRelay {
    // A map of camera IDs to their corresponding broadcast channels.
    streams: Arc<RwLock<HashMap<String, Sender<Bytes>>>>,
    connection_manager: Arc<ConnectionManager>,
}

impl StreamRelay {
    // Creates a new StreamRelay.
    pub fn new(connection_manager: Arc<ConnectionManager>) -> Self {
        Self {
            streams: Arc::new(RwLock::new(HashMap::new())),
            connection_manager,
        }
    }

    // Adds a new stream for a given camera ID.
    pub async fn add_stream(&self, camera_id: String) {
        let mut streams = self.streams.write().await;
        if !streams.contains_key(&camera_id) {
            let (tx, _) = broadcast::channel(100);
            streams.insert(camera_id, tx);
        }
    }

    // Relays a video frame to the appropriate stream.
    pub async fn relay_frame(&self, camera_id: &str, frame: Bytes) {
        if let Some(tx) = self.streams.read().await.get(camera_id) {
            let _ = tx.send(frame);
        }
    }

    // Handles WebSocket connections for streaming to web clients.
    pub async fn websocket_handler(
        ws: WebSocketUpgrade,
        axum::extract::Path(camera_id): axum::extract::Path<String>,
        axum::extract::Extension(relay): axum::extract::Extension<Arc<StreamRelay>>,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Self::handle_socket(socket, camera_id, relay))
    }

    // Handles an individual WebSocket connection.
    async fn handle_socket(socket: WebSocket, camera_id: String, relay: Arc<StreamRelay>) {
        let (mut sender, _receiver): (SplitSink<WebSocket, Message>, SplitStream<WebSocket>) =
            socket.split();

        let mut rx = {
            let streams = relay.streams.read().await;
            streams.get(&camera_id).map(|tx| tx.subscribe())
        };

        if let Some(mut rx) = rx {
            if let Some(connection) = relay.connection_manager.get_connection(&camera_id) {
                while let Ok(frame) = rx.recv().await {
                    if sender.send(Message::Binary(frame)).await.is_err() {
                        break;
                    }
                }
            }
        }
    }
}

// Creates a new Axum router for the stream relay.
pub fn create_router(relay: Arc<StreamRelay>) -> Router {
    Router::new()
        .route("/stream/:camera_id", get(StreamRelay::websocket_handler))
        .layer(axum::extract::Extension(relay))
}
