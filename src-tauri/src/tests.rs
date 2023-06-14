use std::collections::HashMap;

#[cfg(test)]
mod pseudo_legal_move_generation {
  use crate::{pawn_legal_moves, tests::TestBoard, rook_legal_moves, knight_legal_moves, bishop_legal_moves, queen_legal_moves, king_legal_moves}; 
  
  //DEFAULT BOARD TESTS INIZIO
  #[test]
  fn pawn_w() {
    assert_eq!(pawn_legal_moves('P', &8, &TestBoard::default().board, "-".to_owned()).sort(), vec![16, 24].sort());
  }
  #[test]
  fn pawn_b() {
    assert_eq!(pawn_legal_moves('p', &48, &TestBoard::default().board, "-".to_owned()).sort(), vec![40, 32].sort());
  }
  #[test]
  fn rook_w() {
    assert_eq!(rook_legal_moves('R', &0, &TestBoard::default().board), vec![] as Vec<i8>);
  }
  #[test]
  fn rook_b() {
    assert_eq!(rook_legal_moves('r', &56, &TestBoard::default().board), vec![] as Vec<i8>);
  }
  #[test]
  fn knight_w() {
    assert_eq!(knight_legal_moves('N', &1, &TestBoard::default().board).sort(), vec![18, 16].sort());
  }
  #[test]
  fn knight_b() {
    assert_eq!(knight_legal_moves('n', &57, &TestBoard::default().board).sort(), vec![40, 42].sort());
  }
  #[test]
  fn bishop_w() {
    assert_eq!(bishop_legal_moves('B', &2, &TestBoard::default().board), vec![] as Vec<i8>);
  }
  #[test]
  fn bishop_b() {
    assert_eq!(bishop_legal_moves('b', &58, &TestBoard::default().board), vec![] as Vec<i8>);
  }
  #[test]
  fn queen_w() {
    assert_eq!(queen_legal_moves('Q', &3, &TestBoard::default().board), vec![] as Vec<i8>);
  }
  #[test]
  fn queen_b() {
    assert_eq!(queen_legal_moves('q', &59, &TestBoard::default().board), vec![] as Vec<i8>);
  }
  #[test]
  fn king_w() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::default().board, "-".to_owned()), vec![] as Vec<i8>);
  }
  #[test]
  fn king_b() {
    assert_eq!(king_legal_moves('k', &60, &TestBoard::default().board, "-".to_owned()), vec![] as Vec<i8>);
  }

  //DEFAULT BOARD TESTS FINE
  //CENTER PIECE (28) EMPTY BOARD TESTS INIZIO
  
  #[test]
  fn pawn_w_center() {
    assert_eq!(pawn_legal_moves('P', &28, &TestBoard::pawn_w_center_board().board, "-".to_owned()), vec![36]);
  }
  #[test]
  fn pawn_b_center() {
    assert_eq!(pawn_legal_moves('p', &28, &TestBoard::pawn_b_center_board().board, "-".to_owned()), vec![20]);
  }
  #[test]
  fn knight_center() {
    assert_eq!(knight_legal_moves('N', &28, &TestBoard::knight_center_board().board).sort(), vec![43, 45, 34, 38, 18, 22, 11, 13].sort());
  }
  #[test]
  fn bishop_center() {
    assert_eq!(bishop_legal_moves('B', &28, &TestBoard::bishop_center_board().board).sort(), vec![1, 10, 19, 37, 46, 55, 7, 14, 21, 35, 42, 49, 56].sort());
  }
  #[test]
  fn rook_center() {
    assert_eq!(rook_legal_moves('R', &28, &TestBoard::rook_center_board().board).sort(), vec![24, 25, 26, 27, 29, 30, 31, 4, 12, 20, 36, 44, 52, 60].sort());
  }
  #[test]
  fn queen_center() {
    assert_eq!(queen_legal_moves('Q', &28, &TestBoard::queen_center_board().board).sort(), vec![1, 10, 19, 37, 46, 55, 7, 14, 21, 35, 42, 49, 56, 24, 25, 26, 27, 29, 30, 31, 4, 12, 20, 36, 44, 52, 60].sort());
  }
  #[test]
  fn king_center() {
    assert_eq!(king_legal_moves('K', &28, &TestBoard::king_center_board().board, "-".to_owned()).sort(), vec![19, 20, 21, 27, 29, 35, 36, 37].sort());
  }

  //CENTER PIECE (28) EMPTY BOARD TESTS FINE
  //CORNER PIECE (0) EMPTY BOARD TESTS INIZIO

  #[test]
  fn knight_corner() {
    assert_eq!(knight_legal_moves('N', &0, &TestBoard::knight_corner_board().board).sort(), vec![10, 17].sort());
  }

  #[test]
  fn bishop_corner() {
    assert_eq!(bishop_legal_moves('B', &0, &TestBoard::bishop_corner_board().board).sort(), vec![9, 18, 27, 36, 45, 54, 63].sort());
  }

  #[test]
  fn rook_corner() {
    assert_eq!(rook_legal_moves('R', &0, &TestBoard::rook_corner_board().board).sort(), vec![1, 2, 3, 4, 5, 6, 7, 8, 16, 24, 32, 40, 48, 56].sort());
  }

  #[test]
  fn queen_corner() {
    assert_eq!(queen_legal_moves('Q', &0, &TestBoard::queen_corner_board().board).sort(), vec![1, 2, 3, 4, 5, 6, 7, 8, 16, 24, 32, 40, 48, 56, 9, 18, 27, 36, 45, 54, 63].sort());
  }

  #[test]
  fn king_corner() {
    assert_eq!(king_legal_moves('K', &0, &TestBoard::king_corner_board().board, "-".to_owned()).sort(), vec![1, 8, 9].sort());
  }

  //CORNER PIECE (0) EMPTY BOARD TESTS FINE
  //PIECE ALL BLOCKED TESTS INIZIO

  #[test]
  fn pawn_w_single_blocked() {
    assert_eq!(pawn_legal_moves('P', &8, &TestBoard::pawn_w_single_blocked_board().board, "-".to_owned()), vec![] as Vec<i8>);
  }

  #[test]
  fn pawn_b_single_blocked() {
    assert_eq!(pawn_legal_moves('p', &48, &TestBoard::pawn_b_single_blocked_board().board, "-".to_owned()), vec![] as Vec<i8>);
  }

  #[test]
  fn pawn_w_double_blocked() {
    assert_eq!(pawn_legal_moves('P', &8, &TestBoard::pawn_w_double_blocked_board().board, "-".to_owned()), vec![16]);
  }

  #[test]
  fn pawn_b_double_blocked() {
    assert_eq!(pawn_legal_moves('p', &48, &TestBoard::pawn_b_double_blocked_board().board, "-".to_owned()), vec![40]);
  }

  #[test]
  fn knight_blocked() {
    assert_eq!(knight_legal_moves('N', &1, &TestBoard::knight_blocked_board().board), vec![] as Vec<i8>);
  }

  #[test]
  fn bishop_blocked() {
    assert_eq!(bishop_legal_moves('B', &2, &TestBoard::bishop_blocked_board().board), vec![] as Vec<i8>);
  }

  #[test]
  fn rook_blocked() {
    assert_eq!(rook_legal_moves('R', &0, &TestBoard::rook_blocked_board().board), vec![] as Vec<i8>);
  }

  #[test]
  fn queen_blocked() {
    assert_eq!(queen_legal_moves('Q', &3, &TestBoard::queen_blocked_board().board), vec![] as Vec<i8>);
  }

  #[test]
  fn king_blocked() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::king_blocked_board().board, "-".to_owned()), vec![] as Vec<i8>);
  }

  //PIECE ALL BLOCKED TESTS FINE
  //CAPTURES TESTS INIZIO

  #[test]
  fn pawn_w_capture() {
    assert_eq!(pawn_legal_moves('P', &9, &TestBoard::pawn_w_capture_board().board, "-".to_owned()).sort(), vec![16, 18].sort());
  }

  #[test]
  fn pawn_b_capture() {
    assert_eq!(pawn_legal_moves('p', &49, &TestBoard::pawn_b_capture_board().board, "-".to_owned()).sort(), vec![40, 42].sort());
  }

  #[test]
  fn knight_capture() {
    assert_eq!(knight_legal_moves('N', &1, &TestBoard::knight_capture_board().board).sort(), vec![11, 16, 18].sort());
  }

  #[test]
  fn bishop_capture() {
    assert_eq!(bishop_legal_moves('B', &2, &TestBoard::bishop_capture_board().board).sort(), vec![9, 16, 11, 20].sort());
  }

  #[test]
  fn rook_capture() {
    assert_eq!(rook_legal_moves('R', &0, &TestBoard::rook_capture_board().board).sort(), vec![1, 2, 3, 8].sort());
  }

  #[test]
  fn queen_capture() {
    assert_eq!(queen_legal_moves('Q', &3, &TestBoard::queen_capture_board().board).sort(), vec![2, 10, 11, 12, 4].sort());
  }

  #[test]
  fn king_capture() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::king_capture_board().board, "-".to_owned()).sort(), vec![3, 11, 12, 13, 5].sort());
  }

  //CAPTURES TESTS FINE
  //EN PASSANT TESTS INIZIO

  #[test]
  fn pawn_w_en_passant() {
    assert_eq!(pawn_legal_moves('P', &33, &TestBoard::pawn_w_en_passant_board().board, "c6".to_owned()).sort(), vec![41, 42].sort());
  }

  #[test]
  fn pawn_b_en_passant() {
    assert_eq!(pawn_legal_moves('p', &26, &TestBoard::pawn_b_en_passant_board().board, "b3".to_owned()).sort(), vec![17, 18].sort());
  }

  //EN PASSANT TESTS FINE
  //CASTLES TESTS INIZIO

  #[test]
  fn castle_w_king_free() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::castles_free_board().board, "K".to_owned()).sort(), vec![3, 11, 12, 13, 5, 6].sort());
  }

  #[test]
  fn castle_w_queen_free() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::castles_free_board().board, "Q".to_owned()).sort(), vec![3, 11, 12, 13, 5, 2].sort());
  }

  #[test]
  fn castle_w_king_blocked() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::castles_blocked_board().board, "K".to_owned()).sort(), vec![3, 11, 12, 13, 5].sort());
  }

  #[test]
  fn castle_w_queen_blocked() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::castles_blocked_board().board, "Q".to_owned()).sort(), vec![3, 11, 12, 13, 5].sort());
  }

  #[test]
  fn castle_b_king_free() {
    assert_eq!(king_legal_moves('k', &60, &TestBoard::castles_free_board().board, "k".to_owned()).sort(), vec![59, 51, 52, 53, 61, 62].sort());
  }

  #[test]
  fn castle_b_queen_free() {
    assert_eq!(king_legal_moves('k', &60, &TestBoard::castles_free_board().board, "q".to_owned()).sort(), vec![59, 51, 52, 53, 61, 58].sort());
  }

  #[test]
  fn castle_b_king_blocked() {
    assert_eq!(king_legal_moves('k', &60, &TestBoard::castles_blocked_board().board, "k".to_owned()).sort(), vec![59, 51, 52, 53, 61].sort());
  }

  #[test]
  fn castle_b_queen_blocked() {
    assert_eq!(king_legal_moves('k', &60, &TestBoard::castles_blocked_board().board, "q".to_owned()).sort(), vec![59, 51, 52, 53, 61].sort());
  }

  #[test]
  fn castle_w_both_free() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::castles_free_board().board, "KQ".to_owned()).sort(), vec![3, 11, 12, 13, 5, 6, 2].sort());
  }

  #[test]
  fn castle_b_both_free() {
    assert_eq!(king_legal_moves('k', &60, &TestBoard::castles_free_board().board, "kq".to_owned()).sort(), vec![59, 51, 52, 53, 61, 62, 58].sort());
  }

  #[test]
  fn castle_w_both_blocked() {
    assert_eq!(king_legal_moves('K', &4, &TestBoard::castles_blocked_board().board, "KQ".to_owned()).sort(), vec![3, 11, 12, 13, 5].sort());
  }

  #[test]
  fn castle_b_both_blocked() {
    assert_eq!(king_legal_moves('k', &60, &TestBoard::castles_blocked_board().board, "kq".to_owned()).sort(), vec![59, 51, 52, 53, 61].sort());
  }

  //CASTLES TESTS FINE
  //ACCROSS THE BOARD TESTS INIZIO

  #[test]
  fn pawn_w_capture_accross() {
    assert_eq!(pawn_legal_moves('P', &24, &TestBoard::pawn_w_capture_accross_board().board, "-".to_owned()).sort(), vec![32].sort());
  }

  #[test]
  fn pawn_b_capture_accross() {
    assert_eq!(pawn_legal_moves('p', &31, &TestBoard::pawn_b_capture_accross_board().board, "-".to_owned()).sort(), vec![23].sort());
  }

  #[test]
  fn pawn_w_en_passant_accross() {
    assert_eq!(pawn_legal_moves('P', &40, &TestBoard::pawn_w_en_passant_accross_board().board, "h6".to_owned()).sort(), vec![48].sort());
  }

  #[test]
  fn pawn_b_en_passant_accross() {
    assert_eq!(pawn_legal_moves('p', &23, &TestBoard::pawn_b_en_passant_accross_board().board, "a3".to_owned()).sort(), vec![15].sort());
  }

  //ACCROSS THE BOARD TESTS FINE

}


pub struct TestBoard {
  pub board: HashMap<i8, char>,
}

#[allow(dead_code)]
impl TestBoard {
  pub fn default() -> Self { Self { board: HashMap::from([
    (0, 'R'),(1, 'N'),(2, 'B'),(3, 'Q'),(4, 'K'),(5, 'B'),(6, 'N'),(7, 'R'),
    (8, 'P'),(9, 'P'),(10, 'P'),(11, 'P'),(12, 'P'),(13, 'P'),(14, 'P'),(15, 'P'),
    (48, 'p'),(49, 'p'),(50, 'p'),(51, 'p'),(52, 'p'),(53, 'p'),(54, 'p'),(55, 'p'),
    (56 ,'r'),(57, 'n'),(58, 'b'),(59, 'q'),(60, 'k'),(61, 'b'),(62, 'n'),(63, 'r'),
  ]) } }

  pub fn pawn_w_center_board() -> Self { Self { board: HashMap::from([(28, 'P')]) } }
  pub fn pawn_b_center_board() -> Self { Self { board: HashMap::from([(28, 'p')]) } }
  pub fn knight_center_board() -> Self { Self { board: HashMap::from([(28, 'N')]) } }
  pub fn bishop_center_board() -> Self { Self { board: HashMap::from([(28, 'B')]) } }
  pub fn rook_center_board() -> Self { Self { board: HashMap::from([(28, 'R')]) } }
  pub fn queen_center_board() -> Self { Self { board: HashMap::from([(28, 'Q')]) } }
  pub fn king_center_board() -> Self { Self { board: HashMap::from([(28, 'K')]) } }

  pub fn knight_corner_board() -> Self { Self { board: HashMap::from([(0, 'N')]) } }
  pub fn bishop_corner_board() -> Self { Self { board: HashMap::from([(0, 'B')]) } }
  pub fn rook_corner_board() -> Self { Self { board: HashMap::from([(0, 'R')]) } }
  pub fn queen_corner_board() -> Self { Self { board: HashMap::from([(0, 'Q')]) } }
  pub fn king_corner_board() -> Self { Self { board: HashMap::from([(0, 'K')]) } }

  pub fn pawn_w_single_blocked_board() -> Self { Self { board: HashMap::from([(8, 'P'), (16, 'p')]) } }
  pub fn pawn_b_single_blocked_board() -> Self { Self { board: HashMap::from([(48, 'p'), (40, 'P')]) } }
  pub fn pawn_w_double_blocked_board() -> Self { Self { board: HashMap::from([(8, 'P'), (24, 'p')]) } }
  pub fn pawn_b_double_blocked_board() -> Self { Self { board: HashMap::from([(48, 'p'), (32, 'P')]) } }
  pub fn knight_blocked_board() -> Self { Self { board: HashMap::from([(1, 'N'), (16, 'P'), (18, 'P'), (11, 'P')]) } }
  pub fn bishop_blocked_board() -> Self { Self { board: HashMap::from([(2, 'B'), (9, 'P'), (11, 'P')]) } }
  pub fn rook_blocked_board() -> Self { Self { board: HashMap::from([(0, 'R'), (1, 'P'), (8, 'P')]) } }
  pub fn queen_blocked_board() -> Self { Self { board: HashMap::from([(3, 'Q'), (2, 'P'), (10, 'P'), (11, 'P'), (12, 'P'), (4, 'P')]) } }
  pub fn king_blocked_board() -> Self { Self { board: HashMap::from([(4, 'K'), (3, 'P'), (11, 'P'), (12, 'P'), (13, 'P'), (5, 'P')]) } }

  pub fn pawn_w_capture_board() -> Self { Self { board: HashMap::from([(9, 'P'), (16, 'p'), (18, 'p')]) } }
  pub fn pawn_b_capture_board() -> Self { Self { board: HashMap::from([(49, 'p'), (40, 'P'), (42, 'P')]) } }
  pub fn knight_capture_board() -> Self { Self { board: HashMap::from([(1, 'N'), (18, 'p')]) } }
  pub fn bishop_capture_board() -> Self { Self { board: HashMap::from([(2, 'B'), (20, 'p')]) } }
  pub fn rook_capture_board() -> Self { Self { board: HashMap::from([(0, 'R'), (3, 'n'), (8, 'p')]) } }
  pub fn queen_capture_board() -> Self { Self { board: HashMap::from([(3, 'Q'), (2, 'n'), (10, 'p'), (11, 'p'), (12, 'p'), (4, 'b')]) } }
  pub fn king_capture_board() -> Self { Self { board: HashMap::from([(4, 'K'), (3, 'b'), (11, 'p'), (12, 'p'), (13, 'p'), (5, 'n')]) } }

  pub fn pawn_w_en_passant_board() -> Self { Self { board: HashMap::from([(33, 'P'), (34, 'p')]) } }
  pub fn pawn_b_en_passant_board() -> Self { Self { board: HashMap::from([(26, 'p'), (25, 'P')]) } }

  pub fn castles_free_board() -> Self { Self { board: HashMap::from([(4, 'K'), (7, 'R'), (0, 'R'), (60, 'k'), (63, 'r'), (56, 'r')]) } }
  pub fn castles_blocked_board() -> Self { Self { board: HashMap::from([
    (4, 'K'), (7, 'R'), (0, 'R'), (1, 'N'), (6, 'N'),
    (60, 'k'), (63, 'r'), (56, 'r'), (57, 'n'), (62, 'n')
  ]) } }

  pub fn pawn_w_capture_accross_board() -> Self { Self { board: HashMap::from([(24, 'P'), (31, 'p')]) } }
  pub fn pawn_b_capture_accross_board() -> Self { Self { board: HashMap::from([(31, 'p'), (24, 'P')]) } }
  pub fn pawn_w_en_passant_accross_board() -> Self { Self { board: HashMap::from([(40, 'P'), (39, 'p')]) } }
  pub fn pawn_b_en_passant_accross_board() -> Self { Self { board: HashMap::from([(23, 'p'), (24, 'P')]) } }

}