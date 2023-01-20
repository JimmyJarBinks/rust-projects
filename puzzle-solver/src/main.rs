use std::{env, process};

use std::{error::Error, fs};

struct Command {
    puzzle: String,
    file: String,
}

impl Command {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
        args.next();

        let puzzle = match args.next() {
            Some(arg) => arg.to_lowercase(),
            None => return Err("At least 1 argument required to specify puzzle type"),
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => format!("{}.txt", puzzle),
        };

        Ok(Command { puzzle, file })
    }
}

fn run(command: Command) -> Result<(), Box<dyn Error>> {
    println!(
        "Solving Puzzle: {}",
        command.puzzle[0..1].to_uppercase() + &command.puzzle[1..]
    );
    let contents = fs::read_to_string(command.file)?;
    println!("Puzzle:\n{contents}");
    Ok(())
}

fn main() {
    let command = Command::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(err) = run(command) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
}
*/
