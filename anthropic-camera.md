Here is the spec for this project, steps 1,2,3 and 4. Should already be completed.

/home/nicole/Documents/mycorrhizae/flumph/flumph-mobile
However currently the project doesnt pass a compilation check. Could you make sure everything is finished and up to spec with step 4, and that everything compiles?


To help with some of the tricky libraries there is some example code availible to figure out how the imports are structured and how to best implement the library code:
/home/nicole/Documents/mycorrhizae/flumph/examples/nokhwa_webcam_capture_example.rs
/home/nicole/Documents/mycorrhizae/flumph/examples/rav1e_simple_example.rs
/home/nicole/Documents/mycorrhizae/flumph/examples/webrtc_stream_from_disk.rs

(Note: the cargo toml dependencies are fully up to date, its only the commented code that is old. If you need to add a package use `cargo add <packagename>` to ensure the package is as up to date as possible)

There are some compilation bugs that I am working out in 
/home/nicole/Documents/mycorrhizae/flumph/flumph-mobile/src-tauri/src/encoder.rs
and 
/home/nicole/Documents/mycorrhizae/flumph/flumph-mobile/src-tauri/src/camera.rs
for now ignore both of those I can deal with them. Just try to fix the compilation bugs in:
/home/nicole/Documents/mycorrhizae/flumph/flumph-mobile/src-tauri/src/webrtc_client.rs

If you get stuck or encounter some inscrutable bug thats fine just quit out and ask for help :)

make sure to check to make sure the rust actually runs by running check and build commands routinely specifically by using the command
RUSTFLAGS="-A warnings" cargo check --message-format=short
## Prompt 1: Desktop Camera App (Tauri)

### Project Overview
You are building a desktop camera application using Tauri (Rust backend + web frontend) that captures video from a webcam, encodes it using AV1 with rav1e, and streams it to a server via WebRTC. The app must minimize bandwidth usage and provide a control plane for remote management.

### Technical Requirements

**Framework**: Tauri v1.x with Rust backend
**Video Encoding**: rav1e crate for AV1 encoding
**WebRTC**: webrtc crate (https://github.com/webrtc-rs/webrtc)
**Camera Access**: Use platform-specific APIs (V4L2 for Linux, DirectShow for Windows, AVFoundation for macOS)
**Frontend**: Simple HTML/CSS/JavaScript interface
**Configuration**: JSON config file for server IP, encoding settings, etc.

### Architecture Components

1. **Camera Manager**: Handles webcam enumeration, selection, and frame capture
2. **Video Encoder**: Uses rav1e to encode raw frames to AV1
3. **WebRTC Client**: Manages peer connection, sends encoded video stream
4. **Control Channel**: Receives commands via WebRTC data channels
5. **Configuration Manager**: Loads/saves app settings
6. **UI Controller**: Bridges frontend to Rust backend via Tauri commands

### Detailed Implementation Steps

1. **Set up Tauri project structure**
   - Create new Tauri project with Rust backend
   - Configure Cargo.toml with required dependencies: webrtc, rav1e, tokio, serde, tauri
   - Set up basic HTML/CSS/JS frontend with camera preview and controls

2. **Implement Camera Manager (src/camera.rs)**
   - Create struct `CameraManager` with methods for device enumeration
   - Implement platform-specific camera access (use conditional compilation)
   - Add frame capture functionality returning RGB24 frames at 30fps
   - Include error handling for device access failures

3. **Create Video Encoder (src/encoder.rs)**
   - Implement `Av1Encoder` struct using rav1e
   - Configure encoder with low-bandwidth settings (500kbps target, 15fps)
   - Add method to encode RGB frames to AV1 packets
   - Implement dynamic bitrate adjustment based on network conditions

4. **Build WebRTC Client (src/webrtc_client.rs)**
   - Create `WebRTCClient` struct managing peer connection
   - Implement offer/answer exchange via HTTP POST to server
   - Set up video track for sending AV1 encoded frames
   - Add data channel for receiving control commands
   - Handle connection state changes and reconnection logic

5. **Implement Control Channel Handler (src/control.rs)**
   - Create message types for start/stop recording, bitrate changes
   - Implement command processing from WebRTC data channel
   - Add response mechanism to acknowledge commands
   - Include error handling and logging

6. **Create Configuration System (src/config.rs)**
   - Define `AppConfig` struct with server IP, port, encoding settings
   - Implement JSON serialization/deserialization
   - Add default configuration and validation
   - Create Tauri command for updating config from frontend

7. **Build Main Application Loop (src/main.rs)**
   - Set up Tauri app with required commands
   - Initialize camera manager and WebRTC client
   - Create async task for video capture and encoding pipeline
   - Implement graceful shutdown and error recovery

8. **Frontend Implementation (src-tauri/ui/)**
   - Create simple HTML interface with camera preview
   - Add controls for start/stop streaming, server connection
   - Implement JavaScript for calling Tauri commands
   - Add status display for connection state and stream health


### Key Code Patterns to Implement

- Use `tokio::spawn` for async camera capture loop
- Implement `Arc<Mutex<>>` for shared state between components
- Use Tauri's event system for frontend-backend communication
- Implement proper WebRTC signaling state machine
- Add bandwidth monitoring and adaptive bitrate control

