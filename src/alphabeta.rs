use crate::types::*;
use crate::othello::*;

pub struct AlphaBeta {
    max_depth: u8,
}

impl AlphaBeta {
    pub fn new(max_depth: u8) -> AlphaBeta {
        AlphaBeta { max_depth, }
    }
}

#[inline(always)]
fn evaluate(oth: Othello) -> i32 {
    let mut res: i32 = 0;

    let black: BitBoard = oth.get_bitboard(Color::Black);
    let white: BitBoard = oth.get_bitboard(Color::White);

    macro_rules! delta_mask {
        ($mask: expr, $val: expr) => {
            res += $val * ((black & $mask).pop_cnt() as i32 - (white & $mask).pop_cnt() as i32);
        }
    }

    //delta_mask!(0x8100000000000081, 20);
    //delta_mask!(0x7E8181818181817E, 5);
    //delta_mask!(0x007E7E7E7E7E7E00, 1);

    delta_mask!(0x8100000000000081, 100);
    delta_mask!(0x2400810000810024, 10);
    delta_mask!(0x1800008181000018, 5);
    delta_mask!(0x00003C3C3C3C0000, -1);
    delta_mask!(0x003C424242423C00, -2);
    delta_mask!(0x4281000000008142, -20);
    delta_mask!(0x0042000000004200, -50);

    res += 5 * (oth.gen_moves(Color::Black).pop_cnt() - oth.gen_moves(Color::White).pop_cnt()) as i32;

    res
}

#[inline(always)]
fn evaluate_end(oth: Othello) -> i32 {
    let score: (u8, u8) = oth.score();

    if score.0 > score.1 {
        std::i32::MAX
    } else if score.0 < score.1 {
        std::i32::MIN
    } else {
        0
    }
}

fn alphabeta(oth: Othello, mut alpha: i32, mut beta: i32, mut color: Color, mut depth: u8) -> i32 {
    if depth == 0 {
        return evaluate(oth);
    }

    let mut moves = oth.gen_moves(color);

    if moves == 0 {
        color = color.invert();
        depth -= 1;
        if depth == 0 {
            return evaluate(oth);
        }
        moves = oth.gen_moves(color);
        if moves == 0 {
            return evaluate_end(oth);
        }
    }

    let new_color = color.invert();
    depth -= 1;
    match color {
        Color::Black => {
            let mut value: i32 = std::i32::MIN;
            while moves != 0 {
                let new_oth = oth.make_move(color, moves.pop_lsb());
                value = std::cmp::max(value, alphabeta(new_oth, alpha, beta, new_color, depth));
                alpha = std::cmp::max(alpha, value);
                if alpha >= beta { break; }
            }
            return value;
        },
        Color::White => {
            let mut value: i32 = std::i32::MAX;
            while moves != 0 {
                let new_oth = oth.make_move(color, moves.pop_lsb());
                value = std::cmp::min(value, alphabeta(new_oth, alpha, beta, new_color, depth));
                beta = std::cmp::min(beta, value);
                if alpha >= beta { break; }
            }
            return value;
        },
    }
}

impl Player for AlphaBeta {
    fn chose_move(&self, oth: Othello, mut moves: BitBoard, color: Color) -> BitBoard {

        let mut handles = vec![];

        while moves != 0 {
            let m = moves.pop_lsb();
            let o = oth.make_move(color, m);
            let c = color.invert();
            let d = self.max_depth;
            handles.push(std::thread::spawn(move || -> (BitBoard, i32) {
                (m, alphabeta(o, std::i32::MIN, std::i32::MAX, c, d))
            }));
        }

        let mut res: (BitBoard, i32) = handles.remove(0).join().unwrap();
        let mut best: BitBoard = res.0;
        let mut val: i32 = res.1;

        match color {
            Color::Black => {
                for handle in handles {
                    res = handle.join().unwrap();
                    if res.1 > val {
                        best = res.0;
                        val = res.1;
                    }
                }
            },
            Color::White => {
                for handle in handles {
                    res = handle.join().unwrap();
                    if res.1 < val {
                        best = res.0;
                        val = res.1;
                    }
                }
            },
        }

        best
    }
}
