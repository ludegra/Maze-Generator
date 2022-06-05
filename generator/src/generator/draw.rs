use crate::node::Node;

mod rectangle;
mod circle;

pub fn generate_svg(maze: Vec<Node>, method: &str, size: &[u32], cell_size: u32) -> String {
    match method {
        "rectangle" => rectangle::generate_svg(maze, size, cell_size),
        "circle" => circle::generate_svg(maze, size, cell_size),
        _ => panic!("Unknown method"),
    }
}

fn draw_path(fill: Option<&str>, stroke: Option<&str>, path: Vec<String>) -> String {
    let mut params = Vec::new();
    if let Some(fill) = fill {
        params.push(format!("fill=\"{}\"", fill));
    }
    if let Some(stroke) = stroke {
        params.push(format!("stroke=\"{}\"", stroke));
    }
    format!("<path d=\"{}\" {} />", path.join(" "), params.join(" "))
}