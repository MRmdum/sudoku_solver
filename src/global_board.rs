use std::fmt;
use std::ops::{Index, IndexMut};

pub struct GlobalBoard {
    data: [[Vec<u8>; 10]; 10],  // Fixed-size 10x10 array of Vec<u8>
    size: usize,
}

impl GlobalBoard {
    /// Create a new GlobalBoard with given size, all elements initialized to empty Vec<u8>
    pub fn new(input: Vec<Vec<u8>>) -> Self {
        // Initialize a 10x10 board with empty Vec<u8>
        let mut data: [[Vec<u8>; 10]; 10] = Default::default();

        for (x, row) in data.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                if let Some(input_row) = input.get(x) {
                    if let Some(&val) = input_row.get(y) {
                        *cell = vec![val]; // Use the provided value
                    } else {
                        *cell = (1..=9).collect(); // Out of bounds in y -> fill with 1..9
                    }
                } else {
                    *cell = (1..=9).collect(); // Out of bounds in x -> fill with 1..9
                }
            }
        }

        Self { data, size:10 }
    }

    /// Get the size of the board (it's 10x10 in this case)
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get an iterator over all (x, y, value) entries
    pub fn iter(&self) -> GlobalBoardIterator<'_> {
        GlobalBoardIterator {
            board: self,
            x: 0,
            y: 0,
        }
    }
}

// Allow read-access via board[(x,y)]
impl Index<(usize, usize)> for GlobalBoard {
    type Output = Vec<u8>;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.data[idx.0][idx.1]
    }
}

// Allow write-access via board[(x,y)] = val
impl IndexMut<(usize, usize)> for GlobalBoard {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.data[idx.0][idx.1]
    }
}

/// Iterator over GlobalBoard
pub struct GlobalBoardIterator<'a> {
    board: &'a GlobalBoard,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GlobalBoardIterator<'a> {
    type Item = (usize, usize, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.board.size || self.y >= self.board.size {
            return None;
        }

        let result = (self.x, self.y, self.board.data[self.x][self.y].clone());

        // Move to next y, and if we run out of y's, move to the next x
        if self.y + 1 >= self.board.size {
            self.x += 1;
            self.y = 0;
        } else {
            self.y += 1;
        }

        Some(result)
    }
}

impl fmt::Display for GlobalBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "---------------------------------")?;
        for x in 0..self.size-1 {
            write!(f, "|")?;
            for y in 0..self.size-1 {
                // Print each number, ensuring there's a space between them
                write!(f, "{:2} ", self.data[x][y][0])?;
                
                // Add vertical separators between 3x3 subgrids
                if (y + 1) % 3 == 0 && y != self.size - 1 {
                    write!(f, "| ")?;
                }
            }
            writeln!(f)?;  // Start a new line after each row

            // Add horizontal separators between 3x3 subgrids
            if (x + 1) % 3 == 0 && x != self.size - 1 {
                writeln!(f, "---------------------------------")?;  // Print a separator line
            }
        }
        Ok(())
    }
}
