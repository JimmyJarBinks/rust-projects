use crate::Puzzle;

pub struct Nonogram {
    _columns: Vec<Vec<u32>>,
    _rows: Vec<Vec<u32>>,
    board: Vec<Vec<char>>,
}

impl Nonogram {
    fn is_valid(&self) -> bool {
        false
    }

    fn fill_board(&mut self) -> bool {
        false
    }
}

impl Puzzle for Nonogram {
    fn build(contents: &mut String) -> Result<Self, Box<dyn std::error::Error>> {
        let row_col_split: Vec<&str> = contents.split('\n').collect();
        let columns = build_runs(row_col_split[0]);
        let rows = build_runs(row_col_split[1]);
        let board = vec![vec!['\u{25A1}'; rows.len()]; columns.len()];
        Ok(Nonogram {
            _columns: columns,
            _rows: rows,
            board,
        })
    }

    fn solve(&mut self) -> bool {
        self.is_valid() && self.fill_board()
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
