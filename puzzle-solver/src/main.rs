use std::{env, error::Error, fs, process};

const SUDOKU_SIZE: u32 = 9;

enum PuzzleType {
    Sudoku,
}

trait Puzzle {
    fn build(contents: &mut String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn is_valid(&self) -> bool;
    fn solve_puzzle(&self);
}

struct Sudoku {
    board: Vec<Vec<u32>>,
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
                "Failed to read puzzle. For sudoku, ensure there are 81 digits total and no non-digit characters .",
            ))
        }
    }

    fn is_valid(&self) -> bool {
        false
    }

    fn solve_puzzle(&self) {}
}

struct Command {
    puzzle: String,
    filename: String,
}

impl Command {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
        args.next();

        let puzzle = match args.next() {
            Some(arg) => arg.to_lowercase(),
            None => return Err("At least 1 argument required to specify puzzle type."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => format!("{}.txt", puzzle),
        };

        Ok(Command { puzzle, filename })
    }
}

fn sudoku_puzzle(contents: &mut String) -> Result<(), Box<dyn Error>> {
    let sudoku = Sudoku::build(contents)?;
    println!("{:?}", sudoku.board);
    sudoku.solve_puzzle();
    println!("{:?}", sudoku.board);
    Ok(())
}

fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let puzzle: PuzzleType = match command.puzzle.as_str() {
        "sudoku" => PuzzleType::Sudoku,
        _ => return Err(Box::from("Specified puzzle not supported")),
    };

    let mut contents = fs::read_to_string(command.filename)?;
    println!(
        "Solving Puzzle: {}",
        command.puzzle[0..1].to_uppercase() + &command.puzzle[1..]
    );

    match puzzle {
        PuzzleType::Sudoku => sudoku_puzzle(&mut contents),
    }
}

fn main() {
    let command = Command::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Application Error: {err}");
        process::exit(1);
    });
    if let Err(err) = run(command) {
        eprintln!("Application Error: {err}");
        process::exit(1);
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
}
*/
