export type Color = 'white' | 'black';

export type Piece = {
  piece : "pawn" | "rook" | "knight" | "bishop" | "queen" | "king";
  piece_color : Color;
  piece_position: number;
  already_moved: boolean;
}

export type EmptyPiece = Pick<Piece, 'piece_position'>

export type TBoard = {
  last_move: string | null;
  board_position: Array<Piece>;
}

export type BoardSet = Array<Piece | EmptyPiece>;

export const initialBoardSet: BoardSet = [
  {piece: "rook", piece_color: "black", piece_position: 0, already_moved: false},
  {piece: "knight", piece_color: "black", piece_position: 1, already_moved: false},
  {piece: "bishop", piece_color: "black", piece_position: 2, already_moved: false},
  {piece: "queen", piece_color: "black", piece_position: 3, already_moved: false},
  {piece: "king", piece_color: "black", piece_position: 4, already_moved: false},
  {piece: "bishop", piece_color: "black", piece_position: 5, already_moved: false},
  {piece: "knight", piece_color: "black", piece_position: 6, already_moved: false},
  {piece: "rook", piece_color: "black", piece_position: 7, already_moved: false},
  {piece: "pawn", piece_color: "black", piece_position: 8, already_moved: false},
  {piece: "pawn", piece_color: "black", piece_position: 9, already_moved: false},
  {piece: "pawn", piece_color: "black", piece_position: 10, already_moved: false},
  {piece: "pawn", piece_color: "black", piece_position: 11, already_moved: false},
  {piece: "pawn", piece_color: "black", piece_position: 12, already_moved: false},
  {piece: "pawn", piece_color: "black", piece_position: 13, already_moved: false},
  {piece: "pawn", piece_color: "black", piece_position: 14, already_moved: false},
  {piece: "pawn", piece_color: "black", piece_position: 15, already_moved: false},
  {piece_position: 16},{piece_position: 17},{piece_position: 18},{piece_position: 19},{piece_position: 20},{piece_position: 21},{piece_position: 22},{piece_position: 23},
  {piece_position: 24},{piece_position: 25},{piece_position: 26},{piece_position: 27},{piece_position: 28},{piece_position: 29},{piece_position: 30},{piece_position: 31},
  {piece_position: 32},{piece_position: 33},{piece_position: 34},{piece_position: 35},{piece_position: 36},{piece_position: 37},{piece_position: 38},{piece_position: 39},
  {piece_position: 40},{piece_position: 41},{piece_position: 42},{piece_position: 43},{piece_position: 44},{piece_position: 45},{piece_position: 46},{piece_position: 47},
  {piece: "pawn", piece_color: "white", piece_position: 48, already_moved: false},
  {piece: "pawn", piece_color: "white", piece_position: 49, already_moved: false},
  {piece: "pawn", piece_color: "white", piece_position: 50, already_moved: false},
  {piece: "pawn", piece_color: "white", piece_position: 51, already_moved: false},
  {piece: "pawn", piece_color: "white", piece_position: 52, already_moved: false},
  {piece: "pawn", piece_color: "white", piece_position: 53, already_moved: false},
  {piece: "pawn", piece_color: "white", piece_position: 54, already_moved: false},
  {piece: "pawn", piece_color: "white", piece_position: 55, already_moved: false},
  {piece: "rook", piece_color: "white", piece_position: 56, already_moved: false},
  {piece: "knight", piece_color: "white", piece_position: 57, already_moved: false},
  {piece: "bishop", piece_color: "white", piece_position: 58, already_moved: false},
  {piece: "queen", piece_color: "white", piece_position: 59, already_moved: false},
  {piece: "king", piece_color: "white", piece_position: 60, already_moved: false},
  {piece: "bishop", piece_color: "white", piece_position: 61, already_moved: false},
  {piece: "knight", piece_color: "white", piece_position: 62, already_moved: false},
  {piece: "rook", piece_color: "white", piece_position: 63, already_moved: false},
];

export const moveLog: Array<string> = [];