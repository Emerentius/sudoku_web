#[macro_use]
extern crate stdweb;
extern crate sudoku;

use wasm_bindgen::prelude::*;
use stdweb::js_export;
use sudoku::Sudoku;
use web_sys::console;

#[js_export]
fn solve(grid: &str) -> Vec<Vec<u8>> {
    let sudoku = Sudoku::new();
    let mut solutions: Vec<Vec<u8>> = vec![];
    web_sys::console::log_1(&"Before solving...".into());


    let solution = sudoku.solve(grid);
    if !solution.is_none() {
        let solution = solution.unwrap();

        solutions.push(sudoku.format_grid(&solution).as_bytes().to_vec());
    }
    solutions
}
