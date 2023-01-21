use std::{env, error::Error, fs, process, slice::Chunks};

const SUDOKU_SIZE: u32 = 9;

trait Puzzle {
    fn build(contents: &mut String) -> Self;
    fn is_solvable() -> bool;
    fn solve();
}

struct Sudoku {
    board: Vec<Vec<u32>>,
}

impl Puzzle for Sudoku {
    fn build(contents: &mut String) -> Self {
        contents.retain(|c| !c.is_whitespace());
        // TODO: Non-digit error
        let digits: Vec<u32> = contents.chars().filter_map(|c| c.to_digit(10)).collect();
        println!("{:?}", digits); // TEMP
        if digits.len() == SUDOKU_SIZE.pow(2) as usize {
            let board: Vec<Vec<u32>> = digits.chunks(9).map(|x| x.to_vec()).collect();
            println!("{:?}", board);
            Sudoku { board }
        } else {
            //TODO: Incorrect size error
            println!("Failed to read grid.");
            Sudoku {
                board: vec![vec![0; 9]; 9],
            }
        }
    }

    fn is_solvable() -> bool {
        true
    }

    fn solve() {
        todo!()
    }
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

fn solve_sudoku(contents: &mut String) {
    let sudoku = Sudoku::build(contents);
}

fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let mut contents = fs::read_to_string(command.filename)?;
    println!(
        "Solving Puzzle: {}",
        command.puzzle[0..1].to_uppercase() + &command.puzzle[1..]
    );
    match command.puzzle.as_str() {
        "sudoku" => solve_sudoku(&mut contents),
        _ => println!("Not Supported"),
    }
    Ok(())
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
