use rand::seq::SliceRandom;
use rand::Rng;

const SIZE: usize = 9;
const SUBGRID_SIZE: usize = 3;

// fn main() {
//     let grid = generate_sudoku();
//     let puzzle = create_puzzle(grid);
//     print_grid(&puzzle);
// }

// Function to generate a random Sudoku grid
pub fn generate_sudoku() -> Vec<Vec<u8>> {
    let mut grid = vec![vec![0; SIZE]; SIZE];

    // Fill diagonal subgrids first
    for i in 0..SIZE {
        if i % SUBGRID_SIZE == 0 {
            fill_subgrid(&mut grid, i);
        }
    }

    // Fill the remaining cells
    if fill_remaining(&mut grid) {
        grid
    } else {
        // If it fails, return an empty grid
        vec![vec![0; SIZE]; SIZE]
    }
}

// Fill a 3x3 subgrid starting at (row, col)
fn fill_subgrid(grid: &mut Vec<Vec<u8>>, start: usize) {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<u8> = (1..=9).collect();
    nums.shuffle(&mut rng);

    let row_offset = start;
    let col_offset = start;

    for i in 0..SUBGRID_SIZE {
        for j in 0..SUBGRID_SIZE {
            grid[row_offset + i][col_offset + j] = nums[i * SUBGRID_SIZE + j];
        }
    }
}

// Check if a number can be placed at (row, col)
fn can_place_number(grid: &Vec<Vec<u8>>, row: usize, col: usize, num: u8) -> bool {
    // Check the row, column, and 3x3 subgrid
    for c in 0..SIZE {
        if grid[row][c] == num {
            return false;
        }
    }
    for r in 0..SIZE {
        if grid[r][col] == num {
            return false;
        }
    }

    let subgrid_row_start = (row / SUBGRID_SIZE) * SUBGRID_SIZE;
    let subgrid_col_start = (col / SUBGRID_SIZE) * SUBGRID_SIZE;
    for i in 0..SUBGRID_SIZE {
        for j in 0..SUBGRID_SIZE {
            if grid[subgrid_row_start + i][subgrid_col_start + j] == num {
                return false;
            }
        }
    }
    true
}

// Fill the remaining cells in the grid using backtracking
fn fill_remaining(grid: &mut Vec<Vec<u8>>) -> bool {
    for row in 0..SIZE {
        for col in 0..SIZE {
            if grid[row][col] == 0 {
                let mut rng = rand::thread_rng();
                let mut nums: Vec<u8> = (1..=9).collect();
                nums.shuffle(&mut rng);

                for &num in nums.iter() {
                    if can_place_number(grid, row, col, num) {
                        grid[row][col] = num;
                        if fill_remaining(grid) {
                            return true;
                        }
                        grid[row][col] = 0; // Backtrack
                    }
                }
                return false; // No valid number found, backtrack
            }
        }
    }
    true // All cells are filled
}

// Create a puzzle by removing numbers from a solved Sudoku grid
pub fn create_puzzle(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut positions: Vec<(usize, usize)> = (0..SIZE)
        .flat_map(|i| (0..SIZE).map(move |j| (i, j)))
        .collect();
    positions.shuffle(&mut rng);

    let mut removed_count = 0;
    let max_removed = 45; // Remove up to 45 numbers for a medium difficulty puzzle (this can be adjusted)

    for (i, j) in positions {
        if grid[i][j] != 0 {
            let backup = grid[i][j];
            grid[i][j] = 0;
            removed_count += 1;

            // Check if the puzzle still has a unique solution
            if !has_unique_solution(&grid) {
                // If not, restore the number
                grid[i][j] = backup;
                removed_count -= 1;
            }

            if removed_count >= max_removed {
                break;
            }
        }
    }

    grid
}

// Check if the Sudoku puzzle has a unique solution
fn has_unique_solution(grid: &Vec<Vec<u8>>) -> bool {
    let mut grid_copy = grid.clone();
    let solutions = solve_sudoku(&mut grid_copy, 0, 0);
    solutions == 1
}

// Backtracking solver to count the number of solutions
fn solve_sudoku(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    if row == SIZE {
        return 1; // Found one solution
    }

    let (next_row, next_col) = if col == SIZE - 1 {
        (row + 1, 0)
    } else {
        (row, col + 1)
    };

    if grid[row][col] != 0 {
        return solve_sudoku(grid, next_row, next_col);
    }

    let mut solutions = 0;
    for num in 1..=9 {
        if can_place_number(grid, row, col, num) {
            grid[row][col] = num;
            solutions += solve_sudoku(grid, next_row, next_col);
            grid[row][col] = 0;

            if solutions > 1 {
                return solutions; // Early exit if multiple solutions are found
            }
        }
    }

    solutions
}

// Function to print the Sudoku grid
pub fn print_grid(grid: &Vec<Vec<u8>>) {
    for row in grid.iter() {
        for &num in row.iter() {
            print!("{} ", num);
        }
        println!();
    }
}
