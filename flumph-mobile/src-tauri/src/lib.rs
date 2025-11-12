pub mod camera;
pub mod encoder;
pub mod webrtc_client;
pub mod control_api;
pub mod signaling;
pub mod stream_relay;
pub mod webrtc_server;

#[cfg(mobile)]
mod mobile;

#[cfg(mobile)]
pub fn init() {
    mobile::init();
}