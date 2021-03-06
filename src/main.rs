mod types;
mod othello;
mod terminal;
mod alphabeta;

use crate::terminal::*;
use crate::alphabeta::*;

fn main() {
    terminal_play(&AlphaBetaPlayer::new(9), &AlphaBetaPlayer::new(10));
}
