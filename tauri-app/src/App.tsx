import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { register } from "@tauri-apps/plugin-global-shortcut";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [payload, setPayload] = useState();
  const [shortcutDisplay, setShortcutDisplay] = useState("");

  async function registerShortcut() {
    await register("CommandOrControl+Shift+C", (event) => {
      console.log(event);
      if (event.state === "Pressed") {
        // Display the key combination that was pressed for 2 seconds
        setShortcutDisplay(event.shortcut);
        setTimeout(() => {
          setShortcutDisplay("");
        }, 2000);
      }
    });
  }

  useEffect(() => {
    registerShortcut();
    const unlisten = listen("my_event", (event) => {
      setPayload(event.payload);
    });

    // Can also register a shortcut event listener from the Rust side and listen for it here
    const unlistenShortcut = listen("shortcut-event", (event) => {
      setShortcutDisplay(event.payload);
      setTimeout(() => {
        setShortcutDisplay("");
      }, 2000);
    });

    return () => {
      unlisten.then((f) => f());
      unlistenShortcut.then((f) => f());
    };
  }, []);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    const greetMsg = await invoke<string>("greet", { name });
    setGreetMsg(greetMsg);
    await writeText(greetMsg);
    console.log(greetMsg);
  }

  if (payload == undefined) {
    return <div> Waiting... </div>;
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more!</p>

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
      <p>Event Listener: {payload}</p>
      <p>Shortcut Display: {shortcutDisplay}</p>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
