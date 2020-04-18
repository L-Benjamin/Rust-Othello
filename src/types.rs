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
    Black, White,
}

impl Color {
    /*
     * Matches White to Black and Black to White.
     */
    #[inline(always)]
    pub fn invert(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
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
    Empty, Black, White,
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

//#################################################################################################
//
//                                    SCORE TYPE
//
//#################################################################################################

/*
 * The score type, holding the score of a finished game.
 */
pub struct Score(u8, u8);

impl Score {
    /*
     * Creates a new Score struct.
     */
    pub fn new(black_score: u8, white_score: u8) -> Score {
        Score(black_score, white_score)
    }

    /*
     * Returns the score associated to the given color.
     */
    pub fn get(&self, color: Color) -> u8 {
        match color {
            Color::Black => self.0,
            Color::White => self.1,
        }
    }
}
