use wasm_bindgen::prelude::wasm_bindgen;

mod draw;
mod logic;
mod tools;

mod template {
    #[macro_use]
    mod macros;

    pub mod circle;
    pub mod rectangle;
}

#[wasm_bindgen]
pub fn generate(method: &str, size: &[u32], cell_size: u32) -> String {
    let (mut maze, starting_index, ending_index) = match method {
        "rectangle" => template::rectangle::generate(size),
        "circle" => template::circle::generate(size),
        _ => panic!("Unknown method"),
    };

    // println!("{:?}", maze);

    logic::generate(&mut maze, starting_index, ending_index);


    let svg = draw::generate_svg(maze, method, size, cell_size, ending_index);

    svg
}
