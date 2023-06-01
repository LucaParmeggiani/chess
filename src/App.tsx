import { invoke } from "@tauri-apps/api/tauri";
import Board from "./components/Board";
import MoveLog from "./components/MoveLog";
import { initialBoardSet } from "./utils/boardConfig";

function App() {
  return (
    <div className="main-container">
      <Board initialPosition={initialBoardSet}/>
      <MoveLog/>
    </div>
  );
}

export default App;

//npm run tauri dev