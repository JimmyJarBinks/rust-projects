#[cfg(test)]
mod tests {

    use std::vec;

    use crate::{run, sudoku::Sudoku, sudoku_puzzle, Command, Puzzle};

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
    #[test]
    fn test_file_not_found() {
        match run(Command {
            puzzle: "sudoku".to_string(),
            filename: "".to_string(),
        }) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
    #[test]
    fn test_bad_sudoku_file() {
        let mut contents = String::from("12345678901234546002546486206p2345904-234589 45869");
        let sudoku = Sudoku::build(&mut contents).unwrap_err();
        assert_eq!(
            sudoku.to_string(),
            "Failed to read puzzle. For Sudoku, ensure there are 81 digits total."
        )
    }
    #[test]
    fn test_unsolvable_sudoku() {
        let mut contents = String::from_utf8(vec![b'3'; 81]).unwrap();
        let status = sudoku_puzzle(&mut contents).unwrap_err();
        assert_eq!(status.to_string(), "The given sudoku could not be solved.")
    }
    #[test]
    fn test_solvable_sudoku() {
        let mut contents = String::from(
            "600 079 032 000 060 500 209 008 700
             900 305 001 850 000 300 473 001 250
             042 680 900 000 013 427 090 200 600",
        );
        let solution: Vec<Vec<u32>> = vec![
            vec![6, 8, 5, 4, 7, 9, 1, 3, 2],
            vec![7, 3, 4, 1, 6, 2, 5, 9, 8],
            vec![2, 1, 9, 5, 3, 8, 7, 6, 4],
            vec![9, 2, 6, 3, 4, 5, 8, 7, 1],
            vec![8, 5, 1, 7, 2, 6, 3, 4, 9],
            vec![4, 7, 3, 8, 9, 1, 2, 5, 6],
            vec![3, 4, 2, 6, 8, 7, 9, 1, 5],
            vec![5, 6, 8, 9, 1, 3, 4, 2, 7],
            vec![1, 9, 7, 2, 5, 4, 6, 8, 3],
        ];
        let mut sudoku = Sudoku::build(&mut contents).unwrap();
        assert!(sudoku.solve());
        assert_eq!(sudoku.board, solution)
    }
}
