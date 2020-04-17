use crate::othello::*;

//#################################################################################################
//
//                                    BITBOARD TYPE
//
//#################################################################################################

/*
 * The BitBoard type, a 64-bits unsigned integer.
 */
pub type BitBoard = u64;

pub trait BitBoardTrait {
    fn pop_lsb(&mut self) -> BitBoard;
    fn pop_cnt(self) -> u8;
    fn contains(self, x: u8, y: u8) -> bool;
}

impl BitBoardTrait for BitBoard {
    /*
     * Searches for the least significant bit in the BitBoard, returns it and pops it from
     * the BitBoard.
     */
    #[inline(always)]
    fn pop_lsb(&mut self) -> BitBoard {
        let lsb: BitBoard = 1u64 << self.trailing_zeros();
        *self ^= lsb;
        lsb
    }

    /*
     * Does a population count and returns the results as an u8.
     */
    #[inline(always)]
    fn pop_cnt(self) -> u8 {
        self.count_ones() as u8
    }

    /*
     * More high-level method to determine if the point at (x, y) is in the given bitboard.
     */
    fn contains(self, x: u8, y: u8) -> bool {
        1u64 << (x + 8 * y) & self != 0
    }
}

//#################################################################################################
//
//                                        COLOR TYPE
//
//#################################################################################################

/*
 * The Color type, an enum that can either be White or Black.
 */
#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White, Black,
}

impl Color {
    /*
     * Matches White to Black and Black to White.
     */
    #[inline(always)]
    pub fn invert(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

//#################################################################################################
//
//                                        SQUARE TYPE
//
//#################################################################################################

/*
 * The Square type, an enum that contains one of the three possible states a square can be in
 * a game of Othello.
 */
#[derive(Clone, Copy)]
pub enum Square {
    Empty, White, Black,
}

//#################################################################################################
//
//                                    PLAYER TRAIT
//
//#################################################################################################

/*
 * A trait representing a player by it's means of choosing a move.
 */
pub trait Player {
    fn chose_move(&self, oth: Othello, moves: BitBoard, color: Color) -> BitBoard;
}
