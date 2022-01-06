#[macro_use]
extern crate stdweb;
extern crate sudoku;

use stdweb::js_export;
use sudoku::Sudoku;

#[js_export]
fn solve(grid: &str) -> Vec<Vec<u8>> {
    let sudoku = Sudoku::new();

    let solution = sudoku.solve(grid);
    if solution.is_none() {
        return vec![];
    }

    let solution = solution.unwrap();
    let solution_chars: Vec<char> = sudoku.format_grid(&solution).chars().collect();

    if solution_chars.len() != 81 {
        return vec![];
    }

    let mut output: Vec<Vec<u8>> = vec![];
    for i in 0..9 {
        let mut intermediate: Vec<u8> = vec![];
        for j in 0..9 {
            let digit_as_char = solution_chars[(i*9) + j];
            let digit = digit_as_char.to_digit(10);
            if digit.is_none() {
                return vec![];
            }
            let digit = digit.unwrap();
            intermediate.push(digit as u8);
        }
        output.push(intermediate);
    }

    output
}
