#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tests;
mod bit_board;

use std::{collections::HashMap, char};

use bit_board::{BitBoard, Sides, Pieces, Position};

struct Bounds {
  min: i8,
  max: i8
}

const BOARD_BOUNDS: Bounds = Bounds { min: 0, max: 63 };
const WHITE_PAWNS_START: [i8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];
const BLACK_PAWNS_START: [i8; 8] = [48, 49, 50, 51, 52, 53, 54, 55];

#[tauri::command]
fn get_legal_moves(piece: char, piece_position: i8, position: HashMap<i8, char>, en_passant: String, castle_rights: String) -> Vec<i8> {
  let legal_moves: Vec<i8> = match &piece.to_ascii_uppercase() {
    'P' => pawn_legal_moves(piece, &piece_position, &position, en_passant),
    'N' => knight_legal_moves(piece, &piece_position, &position),
    'B' => bishop_legal_moves(piece, &piece_position, &position),
    'R' => rook_legal_moves(piece, &piece_position, &position),
    'Q' => queen_legal_moves(piece, &piece_position, &position),
    'K' => king_legal_moves(piece, &piece_position, &position, castle_rights),
    _ => Vec::new(),
  };

  legal_moves
}

fn pawn_legal_moves(piece: char, piece_position: &i8, position: &HashMap<i8, char>, en_passant: String) -> Vec<i8> {
  let mut moves: Vec<i8> = Vec::new();
  let is_white: bool = piece.is_uppercase();
  let moveset: (i8, i8, i8, i8) = match is_white {
    true => (8, 16, 7, 9),
    false => (-8, -16, -7, -9),
  };
  
  if !position.contains_key(&(piece_position + moveset.0)) {
    moves.push(piece_position + moveset.0);                         //single move
    if !position.contains_key(&(piece_position + moveset.1)) {
      if is_white && WHITE_PAWNS_START.contains(piece_position) || !is_white && BLACK_PAWNS_START.contains(piece_position) {
        moves.push(piece_position + moveset.1);                       //double move
      }
    }
  }

  //capture
  if position.contains_key(&(piece_position + moveset.2)) && piece.is_lowercase() == position.get(&(piece_position + moveset.2)).unwrap().is_lowercase() {
    if ((piece_position % 8) == (piece_position + moveset.2 % 8) + 1) || ((piece_position % 8) == (piece_position + moveset.2 % 8) - 1) {
      moves.push(piece_position + moveset.2);                       //capture
    }
  }

  if position.contains_key(&(piece_position + moveset.3)) && piece.is_lowercase() == position.get(&(piece_position + moveset.3)).unwrap().is_lowercase() {
    if ((piece_position % 8) == (piece_position + moveset.3 % 8) + 1) || ((piece_position % 8) == (piece_position + moveset.3 % 8) - 1) {
      moves.push(piece_position + moveset.3);                       //capture
    }
  }
  
  //en passant
  if en_passant != "-".to_owned() {
    let en_passant_pos = parse_move(&en_passant);
    if en_passant_pos == piece_position + moveset.2 {
      if ((piece_position % 8) == (piece_position + moveset.2 % 8) + 1) || ((piece_position % 8) == (piece_position + moveset.2 % 8) - 1) {
        moves.push(en_passant_pos);                                 //en passant
      }
    }
    if en_passant_pos == piece_position + moveset.3 {
      if ((piece_position % 8) == (piece_position + moveset.3 % 8) + 1) || ((piece_position % 8) == (piece_position + moveset.3 % 8) - 1) {
        moves.push(en_passant_pos);                                 //en passant
      }
    }
  }

  moves
}

fn knight_legal_moves(piece: char, piece_position: &i8, position: &HashMap<i8, char>) -> Vec<i8> {
  let mut moves: Vec<i8> = Vec::new();
  let moveset: Vec<i8> = vec![15, 17, -15, -17, 6, 10, -6, -10];
  let is_white: bool = piece.is_uppercase();

  for (index, possible_move) in moveset.iter().enumerate() {
    if piece_position + possible_move >= BOARD_BOUNDS.min && piece_position + possible_move <= BOARD_BOUNDS.max {
      if index <= 3 {
        if piece_position / 8 == ((piece_position + possible_move) / 8) + 2 || piece_position / 8 == ((piece_position + possible_move) / 8) - 2 {
          match position.get(&(piece_position + possible_move)) {
            Some(piece) => {
              if piece.is_uppercase() != is_white {
                moves.push(piece_position + possible_move);
              }
            },
            None => moves.push(piece_position + possible_move)
          }
        }
      } else {
        if piece_position % 8 == ((piece_position + possible_move) % 8) + 2 || piece_position % 8 == ((piece_position + possible_move) % 8) - 2 {
          match position.get(&(piece_position + possible_move)) {
            Some(piece) => {
              if piece.is_uppercase() != is_white {
                moves.push(piece_position + possible_move);
              }
            },
            None => moves.push(piece_position + possible_move)
          }
        }
      }
    }
  }

  moves
}

fn bishop_legal_moves(piece: char, piece_position: &i8, position: &HashMap<i8, char>) -> Vec<i8> { 
  let mut moves: Vec<i8> = Vec::new();
  let moveset: Vec<i8> = vec![9, -9, 7, -7];
  let is_white: bool = piece.is_uppercase();
  
  for possible_move in moveset {
    let mut sliding_count: i8 = 1;
    loop {
      if piece_position + possible_move * sliding_count > BOARD_BOUNDS.max || piece_position + possible_move * sliding_count < BOARD_BOUNDS.min { break; }
      if (piece_position % 8) + sliding_count != (piece_position + possible_move * sliding_count) % 8 { break; }
      match position.get(&(piece_position + possible_move * sliding_count)) {
        Some(piece) => {
          if piece.is_uppercase() == is_white { break; } else {
            moves.push(piece_position + (possible_move * sliding_count));
            break;
          }
        },
        None => (),
      }

      moves.push(piece_position + (possible_move * sliding_count));
      sliding_count += 1;
      if sliding_count > 7 { break; }
    }
  }

  moves
}

fn rook_legal_moves(piece: char, piece_position: &i8, position: &HashMap<i8, char>) -> Vec<i8> {
  let mut moves: Vec<i8> = Vec::new();
  let is_white = piece.is_uppercase();
  let moveset: Vec<i8> = vec![8, -8, 1, -1];
  
  for (index, possible_move) in moveset.iter().enumerate() {
    let mut sliding_count: i8 = 1;
    loop {
      if piece_position + possible_move * sliding_count > BOARD_BOUNDS.max || piece_position + possible_move * sliding_count < BOARD_BOUNDS.min { break; }
      if index > 2 && (piece_position / 8) == (piece_position + possible_move * sliding_count) / 8 { break; }
      match position.get(&(piece_position + possible_move * sliding_count)) {
        Some(piece) => {
          if piece.is_uppercase() == is_white { break; } else {
            moves.push(piece_position + possible_move * sliding_count);
            break;
          }
        },
        None => (),
      }

      moves.push(piece_position + possible_move * sliding_count);
      sliding_count += 1;
      if sliding_count > 7 { break; }
    }
  }

  moves
}

fn queen_legal_moves(piece: char, piece_position: &i8, position: &HashMap<i8, char>) -> Vec<i8> {
  let mut moves: Vec<i8> = Vec::new();
  moves.append(&mut bishop_legal_moves(piece, &piece_position, &position));
  moves.append(&mut rook_legal_moves(piece, &piece_position, &position));
  moves
}

fn king_legal_moves(piece: char, piece_position: &i8, position: &HashMap<i8, char>, castle_rights: String) -> Vec<i8> {
  let mut moves: Vec<i8> = Vec::new();
  let moveset: Vec<i8> = vec![1, -1, 8, -8, 7, -7, 9, -9];
  let castle_moveset: Vec<i8> = vec![2, -2];
  let is_white = piece.is_uppercase();

  for (index, possible_move) in moveset.iter().enumerate() {
    if piece_position + possible_move >= BOARD_BOUNDS.max && piece_position + possible_move <= BOARD_BOUNDS.min { break; }
    if index <= 1 && (piece_position / 8) == (piece_position + possible_move) / 8 { break; }
    if position.get(&(piece_position + possible_move)).unwrap().is_lowercase() != is_white {
      moves.push(piece_position + possible_move);
    }
    if index > 1 && index <= 3 {
      if position.get(&(piece_position + possible_move)).unwrap().is_lowercase() != is_white {
        moves.push(piece_position + possible_move);
      }
    }
    if index > 3 && piece_position % 8 == (piece_position + possible_move) % 8 {
      if position.get(&(piece_position + possible_move)).unwrap().is_lowercase() != is_white {
        moves.push(piece_position + possible_move);
      }
    }
  }

  //castle rights
  if is_white {
    if castle_rights.contains("K") {
      if free_space_check(true, &position, &piece_position) {
        moves.push(piece_position + castle_moveset[0]);
      }
    }
    if castle_rights.contains("Q") {
      if free_space_check(false, &position, &piece_position) {
        moves.push(piece_position + castle_moveset[1]);
      }
    }
  } else {
    if castle_rights.contains("k") {
      if free_space_check(true, &position, &piece_position) {
        moves.push(piece_position + castle_moveset[0]);
      }
    }
    if castle_rights.contains("q") {
      if free_space_check(false, &position, &piece_position) {
        moves.push(piece_position + castle_moveset[1]);
      }
    }
  }

  moves
}

fn free_space_check (is_king_side: bool, position: &HashMap<i8, char>, piece_position: &i8) -> bool {
  let mut res: bool = false;
  if is_king_side {
    if position.get(&(piece_position + 1)).is_none() && position.get(&(piece_position + 2)).is_none() { res = true; }
  } else {
    if position.get(&(piece_position - 1)).is_none() && position.get(&(piece_position - 2)).is_none() { res = true; }
  }
  res
}

fn parse_move(move_string: &String) -> i8 {
  let coords: Vec<char> = move_string.chars().collect();
  let col_num: Vec<char> = ('a'..='h').into_iter().collect();
  (coords[1].to_digit(10).unwrap() as i8 - 1) * 8 + (col_num.iter().position(|&r| r == coords[0]).unwrap() as i8 + 1)
}

fn main() {
  tauri::Builder::default().invoke_handler(tauri::generate_handler![
    get_legal_moves
    ]).run(tauri::generate_context!()).expect("error while running tauri application");

  let position: Position;
  let white_queens: BitBoard = position.bit_board_pieces[Sides::WHITE][Pieces::QUEEN];
}