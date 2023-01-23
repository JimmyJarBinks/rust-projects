# Puzzle Solver

Do you find yourself wasting time on Sudoku puzzles? Now you can save time and productivity using this program which automatically solves those pesky puzzles for you. I enjoy puzzles like Sudoku, and a puzzle solver sounded like an excellent idea for a command line application. During this project, I also challenged myself to avoid any external libraries (only the std library was used). This arbitrary limitation motivated me to learn and practice some of the fundamentals of Rust. Additionally, I used this project as an opportunity to improve how I structure and test my code and take advantage of many of Rust's useful tools such as [Rustfmt] and [Clippy].

[Rustfmt]: https://github.com/rust-lang/rustfmt
[Clippy]: https://github.com/rust-lang/rust-clippy

# How to Use

This project can be run on the command line using ```cargo run``` followed by 1 or 2 arguments.
* The first argument indicates the type of puzzle to be solved (Currently supports Sudoku only).
* The second argument indicates the input file for the puzzle to be solved. If no argument is provided, the default file is "[puzzle-type].txt" (ex. sudoku.txt).
* The program will then assess whether the puzzle is solvable. If it can solve the puzzle, the solution will be written to a file called "solution.txt".

# Input File Format

The input file can be any type of text file. Different puzzle types will require different formats to be successfully read.

* Sudoku: The file should contain 81 digits representing the grid with 0's representing empty spaces. Whitespace and other non-digit characters will be ignored. The puzzle will be read by row (left to right, top to bottom).  
```
Example: sudoku.txt
600 003 204
040 200 090
008 000 050
009 030 000
000 600 000
306 000 540
803 002 400
000 180 060
165 070 008
```
