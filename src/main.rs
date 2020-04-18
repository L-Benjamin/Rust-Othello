mod types;
mod othello;
mod terminal;
mod alphabeta;

use crate::terminal::*;
use crate::alphabeta::*;

fn main() {
    terminal_play(&AlphaBetaPlayer::new(8), &AlphaBetaPlayer::new(8));
}
