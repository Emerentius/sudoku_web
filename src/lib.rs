#![feature(use_extern_macros)]

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

    let output: Vec<Vec<u8>> = vec![];
    for i in 0..9 {
        let intermediate: Vec<u8> = vec![];
        for j in 0..9 {
            let digit = solution[(i * 9) + j].first().unwrap() as u8;
            intermediate.push(digit);
        }
        output.push(intermediate);
    }

    output
}
