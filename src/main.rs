mod types;
mod othello;
mod terminal;
mod alphabeta;

use crate::terminal::*;
use crate::alphabeta::*;

/*use crate::types::*;
fn print_bb(bb: BitBoard) {
    for y in (0..8).rev() {
        for x in 0..8 {
            if bb & (1u64 << (y*8+x)) != 0 {
                print!("1 ")
            } else {
                print!(". ")
            }
        }
        print!("\n");
    }
    print!("\n");
}*/

fn main() {
    //print_bb(0x00003C3C3C3C0000);

    terminal_play(&AlphaBeta::new(9), &TerminalPlayer);
}
