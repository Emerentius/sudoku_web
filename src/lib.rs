use wasm_bindgen::prelude::*;
use sudoku::Sudoku;

#[wasm_bindgen]
pub fn solve(grid: &str) -> Vec<js_sys::Array> {
    let sudoku = Sudoku::new();
    let mut solutions: Vec<js_sys::Array> = vec![];

    let solution = sudoku.solve(grid);
    if solution.is_some() {
        let solution = solution.unwrap();

        let v = sudoku.format_grid(&solution).as_bytes().to_vec();
        let a: js_sys::Array = v.into_iter().map(JsValue::from).collect();
        solutions.push(a);
    }
    solutions
}
