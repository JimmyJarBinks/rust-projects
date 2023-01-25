mod nonogram;
mod sudoku;
mod tests;

use nonogram::Nonogram;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};
use sudoku::*;

#[derive(Debug, PartialEq)]
pub struct Command {
    puzzle: String,
    filename: String,
}

impl Command {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
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

trait Puzzle {
    fn build(contents: &mut String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn solve(&mut self) -> bool;
    fn format(&self) -> String;
}

#[derive(Debug)]
enum PuzzleType {
    Sudoku,
    Nonogram,
}

fn sudoku_puzzle(contents: &mut String) -> Result<String, Box<dyn Error>> {
    let mut sudoku = Sudoku::build(contents)?;
    match sudoku.solve() {
        true => Ok(sudoku.format()),
        false => Err(Box::from("The given sudoku could not be solved.")),
    }
}

fn nonogram_puzzle(contents: &mut String) -> Result<String, Box<dyn Error>> {
    let mut nonogram = Nonogram::build(contents)?;
    match nonogram.solve() {
        true => Ok(nonogram.format()),
        false => Err(Box::from("The given nonogram could not be solved.")),
    }
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let puzzle: PuzzleType = match command.puzzle.as_str() {
        "sudoku" => PuzzleType::Sudoku,
        "nonogram" => PuzzleType::Nonogram,
        _ => return Err(Box::from("The specified puzzle is not supported.")),
    };

    println!(
        "Solving puzzle: {}\nLooking for file: {}",
        command.puzzle[0..1].to_uppercase() + &command.puzzle[1..],
        command.filename
    );
    let mut contents = fs::read_to_string(command.filename.clone())?;

    let solution = match puzzle {
        PuzzleType::Sudoku => sudoku_puzzle(&mut contents)?,
        PuzzleType::Nonogram => nonogram_puzzle(&mut contents)?,
    };

    println!("{:?} puzzle solved. Writing to solution.txt", puzzle);
    let path = Path::new("solution.txt");
    let mut file = File::create(path)?;
    file.write_all(format!("Solution to puzzle: {}\n", command.filename).as_bytes())?;
    file.write_all(solution.as_bytes())?;

    Ok(())
}
