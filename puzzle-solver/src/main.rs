mod sudoku;

use std::{env, error::Error, fs, process};

use sudoku::*;

trait Puzzle {
    fn build(contents: &mut String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn solve(&mut self);
}

enum PuzzleType {
    Sudoku,
}

#[derive(Debug, PartialEq)]
struct Command {
    puzzle: String,
    filename: String,
}

impl Command {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
        args.next();

        let puzzle = match args.next() {
            Some(arg) => arg.to_lowercase(),
            None => return Err("At least 1 argument is required to specify puzzle type."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => format!("{}.txt", puzzle),
        };

        Ok(Command { puzzle, filename })
    }
}

fn sudoku_puzzle(contents: &mut String) -> Result<(), Box<dyn Error>> {
    let mut sudoku = Sudoku::build(contents)?;
    println!("{:?}", sudoku.board);
    sudoku.solve();
    println!("{:?}", sudoku.board);
    Ok(())
}

fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let puzzle: PuzzleType = match command.puzzle.as_str() {
        "sudoku" => PuzzleType::Sudoku,
        _ => return Err(Box::from("The specified puzzle is not supported.")),
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
        process::exit(0);
    });
    if let Err(err) = run(command) {
        eprintln!("Application Error: {err}");
        process::exit(0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty_command() {
        let args = [String::new(); 0];
        let command = Command::build(args.into_iter());
        assert_eq!(
            command,
            Err("At least 1 argument is required to specify puzzle type.")
        )
    }
    #[test]
    fn test_unsupported_puzzle() {
        let result = run(Command {
            puzzle: "foobar".to_string(),
            filename: "foobar.txt".to_string(),
        })
        .unwrap_err();
        assert_eq!(result.to_string(), "The specified puzzle is not supported.")
    }
    // TODO: Compare errors
    #[test]
    fn test_file_not_found() {
        let _result = run(Command {
            puzzle: "sudoku".to_string(),
            filename: "".to_string(),
        })
        .unwrap_err();
        assert!(false)
        //assert_eq!(result, io::ErrorKind::NotFound)
    }
    // TODO: Compare errors
    #[test]
    fn test_bad_sudoku_file() {
        let mut contents = String::from("12345678901234546002546486206p2345904-234589 45869");
        let _sudoku = Sudoku::build(&mut contents);
        assert!(false)
    }
    // TODO: Test Unsolvable Puzzle
    #[test]
    fn test_unsolvable_sudoku() {
        assert!(false)
    }
}
