Steps 1,2,3,4 and 5 for this project at the path below have already been completed, could you please work on steps 6,7 and 8. And then try to make sure the project is ready to recieve and preview webrtc streams.

/home/nicole/Documents/mycorrhizae/flumph/flumph-server


(Note: the cargo toml dependencies are fully up to date, its only the commented code that is old. If you need to add a package use `cargo add <packagename>` to ensure the package is as up to date as possible)

If you get stuck or encounter some inscrutable bug thats fine just quit out and ask for help :)

make sure to check to make sure the rust actually runs by running check and build commands routinely specifically by using the command
RUSTFLAGS="-A warnings" cargo check --message-format=short

Also in case you find it helpful the file 
/home/nicole/Documents/mycorrhizae/flumph/flumph-server/old_webrtc_example.rs
currently contains an old example of how to get webrtc working.

## Prompt 2: Control and Livestreaming Server (Rust/Axum)

### Project Overview
You are building a server application using Rust and Axum that receives AV1 video streams via WebRTC from camera clients, provides a web interface for viewing the livestream, and implements a control plane for managing connected cameras remotely.

### Technical Requirements

**Framework**: Axum web framework
**WebRTC**: webrtc crate for receiving streams
**Frontend Serving**: Static file serving for HTML/CSS/JS
**Video Streaming**: Raw AV1 stream forwarding without re-encoding
**Control API**: REST endpoints for sending commands to cameras
**Concurrency**: Tokio async runtime

### Architecture Components

1. **WebRTC Server**: Handles incoming peer connections from cameras
2. **Stream Relay**: Forwards AV1 video data to web viewers
3. **Signaling Server**: Manages WebRTC offer/answer exchange
4. **Control API**: REST endpoints for camera management
5. **Web Frontend**: Simple video player interface
6. **Connection Manager**: Tracks active camera connections

### Detailed Implementation Steps

1. **Set up Axum project structure**
   - Create new Rust project with Axum dependencies
   - Configure Cargo.toml with: axum, webrtc, tokio, serde, tower, hyper
   - Create basic project structure with separate modules

2. **Implement WebRTC Server (src/webrtc_server.rs)**
   - Create `WebRTCServer` struct managing peer connections
   - Implement signaling endpoint for offer/answer exchange
   - Set up video track reception for AV1 streams
   - Add data channel for sending control commands to cameras
   - Include connection state tracking and cleanup

3. **Create Stream Relay System (src/stream_relay.rs)**
   - Implement `StreamRelay` struct for forwarding video data
   - Create broadcast mechanism for multiple web viewers
   - Add WebSocket endpoint for streaming to web clients
   - Implement frame buffering and timing for smooth playback
   - Include bandwidth monitoring and stream health checks

4. **Build Signaling Server (src/signaling.rs)**
   - Create REST endpoints for WebRTC signaling (POST /offer, POST /answer)
   - Implement session management for multiple cameras
   - Add camera identification and authentication
   - Include proper CORS handling for web frontend
   - Add error handling and validation

5. **Implement Control API (src/control_api.rs)**
   - Create REST endpoints: POST /cameras/{id}/start, POST /cameras/{id}/stop
   - Add endpoints for bitrate adjustment and configuration
   - Implement command queuing and delivery via WebRTC data channels
   - Add camera status endpoints (GET /cameras, GET /cameras/{id}/status)
   - Include request validation and error responses

6. **Create Web Frontend (static/)**
   - Build HTML page with video player using Media Source Extensions. 
   - Add JavaScript for WebSocket connection to stream relay
   - Create control interface for camera management
   - Implement AV1 decoder setup for browser playback
   - Add connection status and stream health indicators.
   - Use maud for html templating like so:
```rs
use maud::html;

fn main() {
    let best_pony = "Pinkie Pie";
    let numbers = [1, 2, 3, 4];
    let secret_message = "Surprise!";
    let markup = html! {
      p title=(secret_message) { "Hi, " (best_pony) "!" }
          p {
              "I have " (numbers.len()) " numbers, "
              "and the first one is " (numbers[0])
          }
      };
    println!("{}", markup.into_string());
}
```

7. **Build Connection Manager (src/connection_manager.rs)**
   - Create `ConnectionManager` struct tracking active cameras
   - Implement connection registration and cleanup
   - Add heartbeat mechanism for connection health
   - Include connection metrics and logging
   - Add automatic reconnection handling

8. **Implement Main Server (src/main.rs)**
   - Set up Axum router with all endpoints
   - Configure static file serving for web frontend
   - Initialize WebRTC server and connection manager
   - Add graceful shutdown handling
   - Include comprehensive logging setup



### Key Implementation Details

- Use `Arc<RwLock<>>` for shared connection state
- Implement proper WebRTC peer connection lifecycle
- Use Axum's built-in WebSocket support for stream relay
- Create custom AV1 frame parser for stream processing
- Implement connection pooling and resource management


### REST API Endpoints to Implement

- `POST /api/signaling/offer` - WebRTC offer from camera
- `POST /api/signaling/answer` - WebRTC answer to camera
- `GET /api/cameras` - List connected cameras
- `GET /api/cameras/{id}/status` - Camera status
- `POST /api/cameras/{id}/start` - Start camera stream
- `POST /api/cameras/{id}/stop` - Stop camera stream
- `POST /api/cameras/{id}/config` - Update camera settings
- `GET /stream/{camera_id}` - WebSocket stream endpoint

### Frontend Requirements

- Simple HTML5 video player with AV1 support
- WebSocket connection for receiving stream data
- Basic controls for camera management
- Connection status and stream health display
- Responsive design for desktop viewing


If you get stuck or encounter some inscrutable bug thats fine just quit out and ask for help :)

make sure to check to make sure the rust actually runs by running check and build commands routinely specifically by using the command
RUSTFLAGS="-A warnings" cargo check --message-format=short
