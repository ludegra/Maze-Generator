use wasm_bindgen::prelude::wasm_bindgen;

mod template {
    pub mod circle;
    pub mod rectangle;
}

mod draw;
mod logic;

#[wasm_bindgen]
pub fn generate(method: &str, size: &[u32], cell_size: u32) {
    let (mut maze, starting_index, ending_index) = match method {
        "rectangle" => template::rectangle::generate(size),
        "circle" => template::circle::generate(size),
        _ => panic!("Unknown method"),
    };


    logic::generate(&mut maze, starting_index, ending_index);

    let svg = draw::generate_svg(maze, method, size, cell_size);

    println!("{}", svg);
}