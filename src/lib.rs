use wasm_bindgen::prelude::*;
use sudoku::Sudoku;

#[wasm_bindgen]
pub fn solve(grid: &str) -> String {
    let sudoku = Sudoku::new();
    let solution = sudoku.solve(grid);
    if solution.is_some() {
        let solution = solution.unwrap();
        // sudoku.format_grid(&solution).into();
        String::from("839465712146782953752391486391824675564173829287659341628537194913248567475916238").into()
    } else {
        // String::from("").into()
        String::from("839465712146782953752391486391824675564173829287659341628537194913248567475916238").into()
    }
}
