use crate::types::*;
use crate::othello::*;

//#################################################################################################
//
//                                         MACROS
//
//#################################################################################################

/*
 * Below are some useful macros to add some colors to the terminal or clear the screen.
 */

macro_rules! red {
    ($text: expr) => { print!("\x1b[1;31m{}\x1b[0m", $text); }
}

macro_rules! green {
    ($text: expr) => { print!("\x1b[1;32m{}\x1b[0m", $text); }
}

macro_rules! yellow {
    ($text: expr) => { print!("\x1b[1;33m{}\x1b[0m", $text); }
}

macro_rules! cyan {
    ($text: expr) => { print!("\x1b[1;36m{}\x1b[0m", $text); }
}

macro_rules! blue {
    ($text: expr) => { print!("\x1b[1;34m{}\x1b[0m", $text); }
}

macro_rules! clear {
    () => { print!("{esc}[2J{esc}[1;1H", esc = 27 as char); }
}

//#################################################################################################
//
//                                       PRINT OTHELLO
//
//#################################################################################################

/*
 * Prints the othello board to the terminal, with pretty colors.
 */
pub fn print_oth(oth: &Othello, moves: BitBoard, mv: BitBoard) {
    clear!();
    green!("  a b c d e f g h\n");
    for y in 0..8 {
        green!(format!("{} ", y + 1));
        for x in 0..8 {
            match oth.get_square(x, y) {
                Square::White => {
                    if mv.contains(x, y) {
                        red!("O ");
                    } else {
                        cyan!("O ");
                    }
                },
                Square::Black => {
                    if mv.contains(x, y) {
                        red!("X ");
                    } else {
                        blue!("X ");
                    }
                },
                Square::Empty => {
                    if moves.contains(x, y) {
                        yellow!("~ ");
                    } else {
                        print!("- ");
                    }
                }
            }
        }
        println!();
    }
    println!();
}

//#################################################################################################
//
//                                      TERMINAL PLAYER TYPE
//
//#################################################################################################

pub struct TerminalPlayer;

impl Player for TerminalPlayer {
    /*
     * Gets a user input of the form "[a-h][1-8]\n" and verifies it's validity. Converts The
     * input to a BitBoard representing the desired move and checks that it is present in
     * the moves BitBoard.
     */
    fn chose_move(&self, _oth: Othello, moves: BitBoard, color: Color) -> BitBoard {
        let mut input: String = String::new();
        let mut bytes: &[u8];
        let mut mv: BitBoard;

        if color == Color::Black {
            blue!("X player");
        } else {
            cyan!("O player");
        }

        print!(", where do you want to ");
        yellow!("play ");
        println!("? (regex format: \"[a-h][1-8]\\n\")");

        loop {
            input.clear();
            match std::io::stdin().read_line(&mut input) {
                _ => (), // The professional way to handle exceptions.
            }

            bytes = input.as_bytes();
            if bytes.len() != 3 { continue; }
            if bytes[0] < 97 || bytes[1] < 49 { continue; }
            mv = 8 * (bytes[1] as BitBoard - 49) + bytes[0] as BitBoard - 97;
            if mv > 63 { continue; }
            mv = 1u64 << mv;
            if mv & moves != 0 { break; }
        }

        mv
    }
}

//#################################################################################################
//
//                                      TERMINAL PLAY
//
//#################################################################################################

/*
 * Play a game in the terminal, one player taking turn after the other, both being asked
 * what they want to play each time. Gives the score at the end of the game.
 */
pub fn terminal_play(black: &dyn Player, white: &dyn Player) {
    let mut oth: Othello = Othello::new();
    let mut moves: BitBoard;
    let mut mv: BitBoard = 0;
    let mut color: Color = Color::Black;

    loop {
        moves = oth.gen_moves(color);
        if moves == 0 {
            color = color.invert();
            moves = oth.gen_moves(color);
            if moves == 0 { break; }
        }

        print_oth(&oth, moves, mv);

        if color == Color::Black {
            mv = black.chose_move(oth, moves, Color::Black);
        } else {
            mv = white.chose_move(oth, moves, Color::White);
        }

        oth = oth.make_move(color, mv);
        color = color.invert();
    }

    print_oth(&oth, moves, mv);
    let score: Score = oth.score();
    print!("Game over! Final score is [");
    blue!(format!("X: {}", score.get(Color::Black)));
    print!(" - ");
    cyan!(format!("O: {}", score.get(Color::White)));
    println!("]");
    if score.get(Color::Black) > score.get(Color::White) {
        blue!("X player won ! Congatulations !\n");
    } else if score.get(Color::Black) < score.get(Color::White) {
        cyan!("O player won ! Congatulations !\n");
    } else {
        println!("It's a draw !");
    }
    println!();
}

//#################################################################################################
//
//                                      NOSCREEN PLAY
//
//#################################################################################################

/*
 * Play a game with no output on the screen.
 */
pub fn no_screen_play(black: &dyn Player, white: &dyn Player) -> Score {
    let mut oth: Othello = Othello::new();
    let mut moves: BitBoard;
    let mut mv: BitBoard;
    let mut color: Color = Color::Black;

    loop {
        moves = oth.gen_moves(color);
        if moves == 0 {
            color = color.invert();
            moves = oth.gen_moves(color);
            if moves == 0 { break; }
        }

        if color == Color::Black {
            mv = black.chose_move(oth, moves, Color::Black);
        } else {
            mv = white.chose_move(oth, moves, Color::White);
        }

        oth = oth.make_move(color, mv);
        color = color.invert();
    }

    oth.score()
}
