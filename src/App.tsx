import { useState } from "react";
import Board from "./components/Board";
import MoveLog from "./components/MoveLog";

function App() {

  const [moveLogs, setMoveLogs] = useState<Array<string>>([]);

  return (
    <div className="main-container">
      <Board/>
      <MoveLog moveLog={moveLogs}/>
    </div>
  );
}

export default App;

//npm run tauri dev