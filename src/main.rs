mod global_board;
mod generator;

use generator::{generate_sudoku,create_puzzle,print_grid};
use global_board::GlobalBoard;

fn main() {
    

    let grid = generate_sudoku();
    let puzzle = create_puzzle(grid);
    print_grid(&puzzle);
    // let board = GlobalBoard::new(10);
    // println!("{}",board);
}
