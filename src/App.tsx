import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [msg, setMsg] = useState<String>("");

  const getHelloWorld = async () => {
    setMsg(await invoke("get_hello_world"));
  }

  getHelloWorld();

  return (
    <div className="main-container">
      {msg}
    </div>
  );
}

export default App;