use wasm_bindgen::prelude::*;
use sudoku::Sudoku;

#[wasm_bindgen]
pub fn solve(grid: &str) -> Vec<Vec<u8>> {
    let sudoku = Sudoku::new();
    let mut solutions: Vec<Vec<u8>> = vec![];

    let solution = sudoku.solve(grid);
    if !solution.is_none() {
        let solution = solution.unwrap();

        solutions.push(sudoku.format_grid(&solution).as_bytes().to_vec());
    }
    solutions
}
