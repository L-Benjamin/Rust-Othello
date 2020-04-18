use crate::types::*;

//#################################################################################################
//
//                                     MACROS
//
//#################################################################################################

/*
 * Below are some macros to shift a bitboard in a given direction while avoid wrapping from one
 * side to the other.
 */

macro_rules! north_east {
    ($x: ident) => { ($x & 0xFEFEFEFEFEFEFEFE).wrapping_shl(7) }
}

macro_rules! north {
    ($x: ident) => { $x.wrapping_shl(8) }
}

macro_rules! north_west {
    ($x: ident) => { ($x & 0x7F7F7F7F7F7F7F7F).wrapping_shl(9) }
}

macro_rules! west {
    ($x: ident) => { ($x & 0x7F7F7F7F7F7F7F7F).wrapping_shl(1) }
}

macro_rules! east {
    ($x: ident) => { ($x & 0xFEFEFEFEFEFEFEFE).wrapping_shr(1) }
}

macro_rules! south_west {
    ($x: ident) => { ($x & 0x7F7F7F7F7F7F7F7F).wrapping_shr(7) }
}

macro_rules! south {
    ($x: ident) => { $x.wrapping_shr(8) }
}

macro_rules! south_east {
    ($x: ident) => { ($x & 0xFEFEFEFEFEFEFEFE).wrapping_shr(9) }
}

//#################################################################################################
//
//                                    OTHELLO TYPE
//
//#################################################################################################

/*
 * An Othello board only needs two BitBoards. First BitBoard is White's and second is Black's.
 */
#[derive(Clone, Copy)]
pub struct Othello(BitBoard, BitBoard);

impl Othello {
    /*
     * Creates a new Othello board in the starting position.
     */
    pub fn new() -> Othello {
        Othello(0x0000000810000000, 0x0000001008000000)
    }

    /*
     * Creates a new Othello with the given BitBoards.
     */
    #[inline(always)]
    fn create(black: BitBoard, white: BitBoard) -> Othello {
        Othello(black, white)
    }

    /*
     * Returns the BitBoard associated with the color given in argument.
     */
    #[inline(always)]
    pub fn get_bitboard(&self, color: Color) -> BitBoard {
        match color {
            Color::Black => self.0,
            Color::White => self.1,
        }
    }

//#################################################################################################
//
//                                    MOVE GENERATION
//
//#################################################################################################

    /*
     * Generates all legal moves for the given color and returns the result as a
     * BitBoard.
     */
    pub fn gen_moves(&self, playing: Color) -> BitBoard {
        let own: BitBoard = self.get_bitboard(playing);
        let opp: BitBoard = self.get_bitboard(playing.invert());

        let mut t: BitBoard;
        let mut moves: BitBoard = 0;

        macro_rules! search_in_direction {
            ($dir: ident) => {
                t = opp & $dir!(own);
                t |= opp & $dir!(t);
                t |= opp & $dir!(t);
                t |= opp & $dir!(t);
                t |= opp & $dir!(t);
                t |= opp & $dir!(t);
                moves |= $dir!(t);
            }
        }

        search_in_direction!(north_east);
        search_in_direction!(north);
        search_in_direction!(north_west);
        search_in_direction!(west);
        search_in_direction!(south_west);
        search_in_direction!(south);
        search_in_direction!(south_east);
        search_in_direction!(east);

        moves &= !(own | opp);

        moves
    }

//#################################################################################################
//
//                                    MOVE MAKING
//
//#################################################################################################

    /*
     * Makes the given move on the board and returns the new board.
     */
    pub fn make_move(&self, playing: Color, mv: BitBoard) -> Othello {
        let mut own: BitBoard = self.get_bitboard(playing);
        let mut opp: BitBoard = self.get_bitboard(playing.invert());

        let mut t: BitBoard;

        own |= mv;

        macro_rules! change_in_direction {
            ($dir: ident) => {
                t = opp & $dir!(mv);
                if t != 0 {
                    t |= opp & $dir!(t);
                    t |= opp & $dir!(t);
                    t |= opp & $dir!(t);
                    t |= opp & $dir!(t);
                    t |= opp & $dir!(t);
                    if own & $dir!(t) != 0 {
                        opp ^= t;
                        own ^= t;
                    }
                }
            }
        }

        change_in_direction!(north_east);
        change_in_direction!(north);
        change_in_direction!(north_west);
        change_in_direction!(west);
        change_in_direction!(south_west);
        change_in_direction!(south);
        change_in_direction!(south_east);
        change_in_direction!(east);

        if playing == Color::Black {
            Self::create(own, opp)
        } else {
            Self::create(opp, own)
        }
    }

    /*
     * Returns the state of the square at (x, y), where x and y are in 0..8.
     */
    pub fn get_square(&self, x: u8, y: u8) -> Square {
        if self.get_bitboard(Color::Black).contains(x, y) {
            Square::Black
        } else if self.get_bitboard(Color::White).contains(x, y) {
            Square::White
        } else {
            Square::Empty
        }
    }

//#################################################################################################
//
//                                        ACCESSERS
//
//#################################################################################################

    /*
     * Returns the score associated with the given board, that is, a simple count
     * of how many disks each player has.
     */
    pub fn score(&self) -> Score {
        let black_score = self.get_bitboard(Color::Black).pop_cnt();
        let white_score = self.get_bitboard(Color::White).pop_cnt();

        Score::new(black_score, white_score)
    }
}

//#################################################################################################
//
//                                     PERFT TEST
//
//#################################################################################################

/*
 * Performs a perft test
 */
#[cfg(test)]
mod perft_test {
    use super::*;

    // Change depth here.
    const DEPTH: usize = 10;

    /*
     * The perft function in itself, that counts the number of leaf nodes at depth 9.
     */
    fn perft(oth: Othello, color: Color, depth: usize) -> u64 {
        if depth == 0 { return 1; }

        let mut res: u64 = 0;
        let mut moves: BitBoard = oth.gen_moves(color);

        if moves == 0 {
            moves = oth.gen_moves(color.invert());
            if moves == 0 { return 1; }
            return perft(oth, color.invert(), depth-1);
        }

        while moves != 0 {
            res += perft(oth.make_move(color, moves.pop_lsb()), color.invert(), depth-1)
        }

        res
    }

    /*
     * Test functions that carries a perft type test at a specified depth (tunable).
     */
    #[test]
    fn correctnes() {
        let perft_table = vec![
            1, 4, 12, 56, 244, 1396, 8200, 55092,
            390216, 3005288, 24571284, 212258800,
            1939886636, 18429641748, 184042084512
        ];

        assert!(DEPTH < perft_table.len(), "Depth must be at most {}", perft_table.len() - 1);

        let res: u64 = perft(Othello::new(), Color::Black, DEPTH);

        assert_eq!(res, perft_table[DEPTH], "Got an invalid perft value for a depth of {}", DEPTH);
    }
}
