use wasm_bindgen::prelude::*;
use sudoku::Sudoku;

#[wasm_bindgen]
pub fn solve(grid: &str) -> String {
    String::from("1234").into()
    // let sudoku = Sudoku::new();
    // let solution = sudoku.solve(grid);
    // if solution.is_some() {
    //     let solution = solution.unwrap();
    //     sudoku.format_grid(&solution).into()
    // } else {
    //     String::from("").into()
    // }
}
