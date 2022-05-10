use wasm_bindgen::prelude::wasm_bindgen;

mod grid {
    pub mod square;
}

mod maze;

#[wasm_bindgen]
pub fn generate(method: &str, size: &[u32]) {
    let grid = match method {
        "square" => grid::square::generate(size),
        _ => panic!("Unknown method"),
    };

    let maze = maze::generate(&grid, 0);
}