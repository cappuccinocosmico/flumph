use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;

mod control_api;
mod signaling;
mod stream_relay;
mod webrtc_server;
mod connection_manager;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let connection_manager = Arc::new(connection_manager::ConnectionManager::new());
    let webrtc_server = Arc::new(webrtc_server::WebRTCServer::new(connection_manager.clone()));
    let stream_relay = Arc::new(stream_relay::StreamRelay::new(connection_manager.clone()));

    let app = Router::new()
        .nest("/", control_api::create_router(webrtc_server.clone()))
        .nest("/", signaling::create_router(webrtc_server.clone()))
        .nest("/", stream_relay::create_router(stream_relay.clone()))
        .nest_service("/", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::from_std(std::net::TcpListener::bind(addr).unwrap()).unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}


