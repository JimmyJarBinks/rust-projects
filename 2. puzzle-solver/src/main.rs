use puzzle_solver::*;
use std::{env, process};

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
