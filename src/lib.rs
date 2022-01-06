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

    let mut solutions: Vec<Vec<u8>> = vec![];
    solutions.push(sudoku.format_grid(&solution).as_bytes().to_vec());
    solutions
}
