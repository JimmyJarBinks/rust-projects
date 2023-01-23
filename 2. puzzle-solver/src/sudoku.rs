use std::error::Error;

use crate::Puzzle;

pub const SUDOKU_SIZE: u32 = 9;

#[derive(Debug)]
pub struct Sudoku {
    pub board: Vec<Vec<u32>>,
}

impl Sudoku {
    fn safe_row(&self, row: usize, col: usize, candidate: u32) -> bool {
        let intersecting_val = self.board[row].iter().position(|&x| x == candidate);
        (Option::is_none(&intersecting_val)) || (intersecting_val == Some(col))
    }

    fn safe_col(&self, row: usize, col: usize, candidate: u32) -> bool {
        self.board
            .iter()
            .filter(|row| row[col] == candidate)
            .count()
            == 0
            || self.board[row][col] == candidate
    }

    fn safe_matrix(&self, row: usize, col: usize, candidate: u32) -> bool {
        let (start_row, start_col) = (row - row % 3, col - col % 3);
        for r in start_row..(start_row + 3) {
            for c in start_col..(start_col + 3) {
                if self.board[r][c] == candidate && r != row && c != col {
                    return false;
                }
            }
        }
        true
    }

    fn safe_space(&self, row: usize, col: usize, candidate: u32) -> bool {
        let row_safe = self.safe_row(row, col, candidate);
        let col_safe = self.safe_col(row, col, candidate);
        let matrix_safe = self.safe_matrix(row, col, candidate);
        row_safe && col_safe && matrix_safe
    }

    fn is_valid(&self) -> bool {
        for row in 0..SUDOKU_SIZE as usize {
            for col in 0..SUDOKU_SIZE as usize {
                let filled = self.board[row][col];
                if !self.safe_space(row, col, filled) && filled != 0 {
                    return false;
                }
            }
        }
        true
    }

    // Backtracking Algorithm
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

    fn solve(&mut self) -> bool {
        if self.is_valid() && self.fill_board(0, 0) {
            println!("Sudoku puzzle solved. Writing to solution.txt");
            return true;
        }
        false
    }

    fn format(&self) -> String {
        self.board
            .iter()
            .map(|row| format!("{:?}", row))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
