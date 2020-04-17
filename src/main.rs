mod types;
mod othello;
mod terminal;
mod alphabeta;

use crate::terminal::*;
use crate::alphabeta::*;

fn main() {
    terminal_play(&TerminalPlayer, &AlphaBetaPlayer::new(10));
}
