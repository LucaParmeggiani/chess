import '../css/tile.css';
import { Color, EmptyPiece, Piece } from '../utils/boardConfig';

export type TileProps = {
    piece: Piece | EmptyPiece
    tileColor: Color
    getLegalMoves: (piece: Piece) => void
    legalMove: boolean
}

const Tile = ({tileColor, piece, getLegalMoves, legalMove} : TileProps) => {
    return (
        <div className="tile" style={{backgroundColor: tileColor === "black" ? "#bbb" : "white"}}>
            { legalMove ? <div className="legal"></div> : null }
            { 'piece' in piece ? <img className="piece" src={`${piece.piece}${piece.piece_color}.svg`} onClick={() => getLegalMoves(piece as Piece)}/> : null }
        </div>
    )
}

export default Tile;