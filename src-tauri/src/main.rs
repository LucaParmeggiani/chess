#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Piece {
  piece: String,
  piece_color: String,
  piece_position: i8,
  already_moved: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Board {
  last_move: Option<String>,
  board_position: Vec<Piece>,
}

#[tauri::command]
fn get_legal_moves(piece: Piece, board: Board) -> Vec<i8> {
  let mut legal_moves: Vec<i8> = vec![];

  println!("piece -> {:?}", piece.piece);
  println!("piece_color -> {:?}", piece.piece_color);
  println!("piece_position -> {:?}", piece.piece_position);
  println!("already_moved -> {:?}", piece.already_moved);
  println!("last_move -> {:?}", board.last_move);
  println!("board_position -> {:?}", board.board_position);

  match piece.piece.as_str() {
    "pawn" => pawn_moves(&mut legal_moves),
    "rook" => rook_moves(&mut legal_moves),
    "knight" => knight_moves(&mut legal_moves),
    "bishop" => bishop_moves(&mut legal_moves),
    "queen" => queen_moves(&mut legal_moves),
    "king" => king_moves(&mut legal_moves),
    _ => vec![],
  }
}

fn pawn_moves(vec: &mut Vec<i8>) -> Vec<i8> {
  let mut legal_moves: Vec<i8> = vec![];
  todo!()
}

fn rook_moves(vec: &mut Vec<i8>) -> Vec<i8> {
  let mut legal_moves: Vec<i8> = vec![];
  todo!()
}

fn knight_moves(vec: &mut Vec<i8>) -> Vec<i8> {
  let mut legal_moves: Vec<i8> = vec![];
  todo!()
}

fn bishop_moves(vec: &mut Vec<i8>) -> Vec<i8> {
  let mut legal_moves: Vec<i8> = vec![];
  todo!()
}

fn queen_moves(vec: &mut Vec<i8>) -> Vec<i8> {
  let mut legal_moves: Vec<i8> = vec![];
  todo!()
}

fn king_moves(vec: &mut Vec<i8>) -> Vec<i8> {
  let mut legal_moves: Vec<i8> = vec![];
  todo!()
}

fn main() {
  tauri::Builder::default().invoke_handler(tauri::generate_handler![
    get_legal_moves
    ]).run(tauri::generate_context!()).expect("error while running tauri application");
}