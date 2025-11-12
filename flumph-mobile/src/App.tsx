import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [cameraMsg, setCameraMsg] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function testCamera() {
    try {
      setCameraMsg("Testing camera...");
      const result = await invoke("test_camera");
      setCameraMsg(result as string);
    } catch (error) {
      setCameraMsg("Error: " + error);
    }
  }

  return (
    <main className="container">
      <h1>Mobile Camera Streamer</h1>
      <p>Flumph Mobile: AV1 streaming with WebRTC</p>

      <div className="row">
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      {/* Camera testing section */}
      <div className="camera-controls">
        <button onClick={testCamera}>Test Camera</button>
        <p>{cameraMsg}</p>
      </div>

      {/* Original greeting section - keep for now */}
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
