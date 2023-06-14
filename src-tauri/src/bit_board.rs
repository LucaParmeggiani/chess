#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Default)]
pub struct BitBoard(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
  bit_board_sides: [BitBoard; 2],
  bit_board_pieces: [[BitBoard; 6]; 2],
}

pub struct Sides;
impl Sides {
  pub const WHITE: usize = 0;
  pub const BLACK: usize = 1;
}
pub struct Pieces;
impl Pieces {
  pub const PAWN: usize = 0;
  pub const BISHOP: usize = 1;
  pub const KNIGHT: usize = 2;
  pub const ROOK: usize = 3;
  pub const QUEEN: usize = 4;
  pub const KING: usize = 5;   
}