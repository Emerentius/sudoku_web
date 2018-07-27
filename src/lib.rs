#![feature(use_extern_macros)]

#[macro_use]
extern crate stdweb;
extern crate sudoku;

use stdweb::js_export;
use sudoku::Sudoku;

#[js_export]
fn solve(string: &str) -> Vec<Vec<u8>> {
    let sudoku = Sudoku::from_str_line(string);

    sudoku.ok()
        .map_or(vec![], |sud| sud.solve_at_most(2))
        .into_iter()
        .map(|solution|
            solution.to_bytes().to_vec()
        )
        .collect()
}
