use crate::node::Node;

mod rectangle;
mod circle;

pub fn generate_svg(maze: Vec<Node>, method: &str, size: &[u32], cell_size: u32, ending_index: usize) -> String {
    match method {
        "rectangle" => rectangle::generate_svg(maze, size, cell_size),
        "circle" => circle::generate_svg(maze, size, cell_size, ending_index),
        _ => panic!("Unknown method"),
    }
}

fn draw_path(fill: Option<&str>, stroke: Option<&str>, path: Vec<String>) -> String {
    let mut params = Vec::new();
    if let Some(fill) = fill {
        params.push(format!("fill=\"{}\"", fill));
    }
    else {
        params.push("fill=\"none\"".to_string());
    }

    if let Some(stroke) = stroke {
        params.push(format!("stroke=\"{}\"", stroke));
    }
    else {
        params.push("stroke=\"none\"".to_string());
    }
    format!("<path d=\"{}\" {} />", path.join(" "), params.join(" "))
}