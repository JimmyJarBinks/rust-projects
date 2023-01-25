use crate::Puzzle;

const EMPTY: char = '\u{25A1}';
const FILLED: char = '\u{25A0}';

pub struct Nonogram {
    columns: Vec<Vec<u32>>,
    rows: Vec<Vec<u32>>,
    board: Vec<Vec<char>>,
}

impl Nonogram {
    fn count<'a>(&self, comparison: impl Iterator<Item = (&'a u32, &'a u32)>) -> usize {
        comparison.filter(|&(a, b)| a >= b).count()
    }

    fn count_end<'a>(&self, comparison: impl Iterator<Item = (&'a u32, &'a u32)>) -> usize {
        comparison.filter(|&(a, b)| a == b).count()
    }

    fn safe_row(&self, row: usize, col: usize) -> bool {
        let mut proposed_row: Vec<u32> = Vec::new();
        for i in 0..self.columns.len() {
            if self.board[row][i] == FILLED {
                if i == 0 || self.board[row][i - 1] == EMPTY {
                    proposed_row.push(1);
                } else {
                    *(proposed_row.last_mut().unwrap()) += 1;
                }
            }
        }
        let actual_row = self.rows[row].to_owned();
        let comparison = actual_row.iter().zip(&proposed_row);
        if col == self.columns.len() - 1 {
            actual_row.len() == proposed_row.len() && self.count_end(comparison) == actual_row.len()
        } else {
            actual_row.len() >= proposed_row.len() && self.count(comparison) == proposed_row.len()
        }
    }

    fn safe_col(&self, row: usize, col: usize) -> bool {
        let mut proposed_col: Vec<u32> = Vec::new();
        for i in 0..self.rows.len() {
            if self.board[i][col] == FILLED {
                if i == 0 || self.board[i - 1][col] == EMPTY {
                    proposed_col.push(1);
                } else {
                    *(proposed_col.last_mut().unwrap()) += 1;
                }
            }
        }
        let actual_col = self.columns[col].to_owned();
        let comparison = actual_col.iter().zip(&proposed_col);
        if row == self.rows.len() - 1 {
            actual_col.len() == proposed_col.len() && self.count_end(comparison) == actual_col.len()
        } else {
            actual_col.len() >= proposed_col.len() && self.count(comparison) == proposed_col.len()
        }
    }

    fn safe_space(&self, row: usize, col: usize) -> bool {
        self.safe_row(row, col) && self.safe_col(row, col)
    }

    fn fill_board(&mut self, mut row: usize, mut col: usize) -> bool {
        let (width, height) = (self.columns.len(), self.rows.len());
        if col == width {
            if row == height - 1 {
                return true;
            }
            row += 1;
            col = 0;
        }
        self.board[row][col] = FILLED;
        if self.safe_space(row, col) && self.fill_board(row, col + 1) {
            return true;
        }
        self.board[row][col] = EMPTY;
        if self.safe_space(row, col) && self.fill_board(row, col + 1) {
            return true;
        }
        false
    }
}

impl Puzzle for Nonogram {
    fn build(contents: &mut String) -> Result<Self, Box<dyn std::error::Error>> {
        let row_col_split: Vec<&str> = contents.split('\n').collect();
        let columns = build_runs(row_col_split[0]);
        let rows = build_runs(row_col_split[1]);
        let board = vec![vec![EMPTY; rows.len()]; columns.len()];
        Ok(Nonogram {
            columns,
            rows,
            board,
        })
    }

    fn solve(&mut self) -> bool {
        self.fill_board(0, 0)
    }

    fn format(&self) -> String {
        self.board
            .iter()
            .map(|row| row.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn build_runs(split: &str) -> Vec<Vec<u32>> {
    let str_runs: Vec<&str> = split.split_whitespace().collect();
    let mut num_runs: Vec<Vec<u32>> = Vec::new();
    for run in str_runs {
        let comma_split: String = run.split(',').collect();
        num_runs.push(comma_split.chars().filter_map(|c| c.to_digit(10)).collect());
    }
    num_runs
}
