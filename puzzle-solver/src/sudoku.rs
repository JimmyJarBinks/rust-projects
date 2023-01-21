use std::error::Error;

use crate::Puzzle;

pub const SUDOKU_SIZE: u32 = 9;

pub struct Sudoku {
    pub board: Vec<Vec<u32>>,
}

impl Sudoku {
    fn safe_space(&self, row: usize, col: usize, candidate: u32) -> bool {
        let rows_safe = !self.board[row].contains(&candidate);
        let columns_safe = self
            .board
            .iter()
            .filter(|row| row[col] == candidate)
            .count()
            == 0;
        // TODO: Reduce matrix_safe code
        let mut matrix_safe = true;
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for r in 0..3 {
            for c in 0..3 {
                if self.board[r + start_row][c + start_col] == candidate {
                    matrix_safe = false;
                    break;
                }
            }
        }
        rows_safe && columns_safe && matrix_safe
    }

    fn fill_board(&mut self, mut row: usize, mut col: usize) -> bool {
        let max = SUDOKU_SIZE as usize;
        if col == max {
            if row == max - 1 {
                return true;
            }
            row += 1;
            col = 0;
        }
        if self.board[row][col] > 0 {
            return self.fill_board(row, col + 1);
        }
        for n in 1..=max {
            let candidate = n as u32;
            if self.safe_space(row, col, candidate) {
                self.board[row][col] = candidate;
                if self.fill_board(row, col + 1) {
                    return true;
                }
            }
            self.board[row][col] = 0;
        }
        false
    }
}

impl Puzzle for Sudoku {
    fn build(contents: &mut String) -> Result<Self, Box<dyn Error>> {
        contents.retain(|c| !c.is_whitespace());
        let digits: Vec<u32> = contents.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() == SUDOKU_SIZE.pow(2) as usize {
            Ok(Sudoku {
                board: digits
                    .chunks(9)
                    .map(|x| x.to_vec())
                    .collect::<Vec<Vec<u32>>>(),
            })
        } else {
            Err(Box::from(
                "Failed to read puzzle. For sudoku, ensure there are 81 digits total.",
            ))
        }
    }

    // Currently uses a backtracking algorithm
    fn solve(&mut self) {
        let success = self.fill_board(0, 0);
        println!("{success}");
    }
}
