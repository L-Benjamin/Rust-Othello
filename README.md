# Rust-Othello

Here is an implementation of the 2 players board game Othello (Also known as Reversi). The rules are availabel on [Wikipedia page of Reversi](https://en.wikipedia.org/wiki/Reversi).

## Move generation

Move generation and move making are carried out thanks to a bitboards-based representation of the game and the dumb7fill algorithm, adapted from chess (See the page of the algorithm on the [chess programming wiki](https://www.chessprogramming.org/Dumb7Fill)).

## Perft

The accuracy of the move engine is tested with the perft method, which consists in counting the numbers of leaf nodes of the game tree at a given limited depth and comparing that number with already-established tables. Here is the perft table used in this project:

|depth|Number of leaf nodes|
|-----|--------------------|
|0    |1                   |
|1    |4                   |
|2    |12                  |
|3    |56                  |
|4    |244                 |
|5    |1396                |
|6    |8200                |
|7    |55092               |
|8    |390216              |
|9    |3005288             |
|10   |24571284            |
|11   |212258800           |
|12   |1939886636          |
|13   |18429641748         |

The value of perft(13) was calculated by this program and never confirmed anywhere else.

## AI

The AI is a simple, yet effective, implementation of a minimax algorithm using alpha-beta pruning. The evaluation function takes both position of the disks and mobility into account.
