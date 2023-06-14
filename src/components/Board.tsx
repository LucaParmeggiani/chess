import "../css/board.css";
import { useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export let legalMoves: Array<number> = [];

const piecesMap = new Map([
    ["p", "pawn"],
    ["r", "rook"],
    ["n", "knight"],
    ["b", "bishop"],
    ["q", "queen"],
    ["k", "king"],
]);

const Board = () => {

    // const [FENBoard, setFENBoard] = useState<string>("8/8/8/4p1K1/2k1P3/8/8/8 w KQkq - 0 1");
    // const [FENBoard, setFENBoard] = useState<string>("4k2r/6r1/8/8/8/8/3R4/R3K3 w KQkq - 0 1");
    const [FENBoard, setFENBoard] = useState<string>("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    const [legalMoves, setLegalMoves] = useState<Array<number>>([]);

    const isLegalMove = (pos: number): JSX.Element | null => {
        return legalMoves.includes(pos) ? <div className="legalMove"></div> : null
    }

    const emptyBoard = useCallback((): Array<boolean> => {
        const res: Array<boolean> = [];
        for (let i = 0; i < 8; i++) {
            for (let j = 0; j < 8; j++) {
                res.push((i + j) % 2 !== 0);
            }
        }
        return res;
    }, [legalMoves]);

    const fillBoard = useCallback((): Array<JSX.Element> => {
        const pieces: Array<JSX.Element> = [];
        FENBoard.split(" ")[0].split("/").forEach((row, i) => {
            const top = i * 100;
            let left: number = 0;
            row.split("").forEach((piece, j) => {
                if (!isNaN(Number(piece))){
                    left += Number(piece) * 100;
                } else {
                    const isLight: boolean = piece.toUpperCase() === piece;
                    pieces.push(
                        <div className="piece" key={piece + i + j} style={{top: `${top}px`, left: `${left}px`}} onClick={() => getLegalMoves(piece, i, j)}>
                            <img src={`${piecesMap.get(piece.toLowerCase())}${isLight ? "white" : "black"}.svg`}/>
                        </div>
                    );
                    left += 100;
                }
            })
        })
        return pieces;
    }, [FENBoard]);

    const getLegalMoves = async (piece: string, i: number, j: number) => {
        setLegalMoves(await invoke("get_legal_moves", {piece, i, j, FENBoard}));
    }

    return (
        <div className="board-container">
            {
                emptyBoard().map((isLight: boolean, i: number) => {
                    return <div className="tile" style={{backgroundColor: isLight ? "#F0D1BA" : "#9F6E5A"}} key={i}>{ isLegalMove(i) }</div>
                })
            }{ fillBoard().map((value: JSX.Element) => { return value; }) }
        </div>
    )
}

export default Board;