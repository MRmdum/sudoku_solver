mod global_board;
mod generator;

use generator::{generate_sudoku,create_puzzle};
use global_board::GlobalBoard;
// Check https://github.com/stonkie/SudokuSolverV1 for simple rules implementation

// TODO : stockage sur dique + génération plusieurs board
// TODO : initialisation vec de possibilité par plateau aussi
// TODO : filtrage vec de possibilité sur ligne/col + plateaux
// TODO : filtrage cases par intersection de vec
// TODO : implémentation placement chiffre

fn main() {
    

    let grid = generate_sudoku();
    let puzzle = create_puzzle(grid);
    let board = GlobalBoard::new(puzzle);
    println!("{}",board);
}
