#[macro_use]
extern crate stdweb;
extern crate sudoku;
extern crate web_sys;
extern crate wasm_bindgen;

use stdweb::js_export;
use sudoku::Sudoku;

#[js_export]
fn solve(grid: &str) -> Vec<Vec<u8>> {
    let sudoku = Sudoku::new();
    let mut solutions: Vec<Vec<u8>> = vec![];
    web_sys::console::log_1(&"Before solving...".into());


    let solution = sudoku.solve(grid);
    web_sys::console::log_1(&"After solving...".into());
    if !solution.is_none() {
        web_sys::console::log_1(&"Solution not none!".into());
        let solution = solution.unwrap();
        web_sys::console::log_1(&"Solution unwrapped!".into());

        solutions.push(sudoku.format_grid(&solution).as_bytes().to_vec());
        web_sys::console::log_1(&"Solution converted and pushed!".into());
    }
    solutions
}
