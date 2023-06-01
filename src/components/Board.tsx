import { useState } from "react";
import "../css/board.css";
import { TBoard, BoardSet, Color, Piece, moveLog } from "../utils/boardConfig";
import Tile from "./Tile";
import { invoke } from "@tauri-apps/api/tauri";

type BoardProps = {
    initialPosition: BoardSet
}

export let legalMoves: Array<number> = [];

const Board = ({initialPosition}: BoardProps) => {

    const [boardPosition, setBoardPosition] = useState<BoardSet>(initialPosition);
    const [legalMoves, setLegalMoves] = useState<Array<number>>([]);

    const getTileColor = (index: number) : Color => {
        if (Math.floor(index / 8) % 2 === 0)
            return index % 2 === 0 ? "black" : "white";
        else return index % 2 === 0 ? "white" : "black";
    }

    const getLegalMoves = async (piece: Piece) => {
        const board: TBoard = {
            last_move: moveLog.slice(-1).pop() || null,
            board_position: boardPosition.filter((pos): pos is Piece => pos.hasOwnProperty('piece'))
        };

        setLegalMoves(await invoke("get_legal_moves", { piece, board }));
    }

    return (
        <div className="board-container">
            {boardPosition.map((tilePiece, index) => {
                return <Tile
                    tileColor={getTileColor(index)}
                    piece={tilePiece}
                    key={index}
                    getLegalMoves={getLegalMoves}
                    legalMove={legalMoves.includes(index)}
                    />
            })}
        </div>
    )
}

export default Board;