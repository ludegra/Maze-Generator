use crate::{
    generator::tools::nodes_in_layer,
    node::{Direction, Node},
};

use super::draw_path;

pub fn generate_svg(maze: Vec<Node>, size: &[u32], cell_size: u32, ending_index: usize) -> String {
    let mut size_iter = size.into_iter();

    let radius = *size_iter.next().unwrap() as usize;
    let intensity = *size_iter.next().unwrap() as usize;

    let mut nodes = maze.into_iter();
    let mut children = Vec::<String>::new();

    let _center = nodes.next().unwrap();

    let mut index = 1;

    for layer in 1..radius {
        let layer_size = nodes_in_layer(layer, intensity);

        for i in 0..layer_size {
            let node = nodes.next().unwrap();

            for connection in node.get_connections() {
                if !connection.active {
                    continue;
                }

                match connection.direction {
                    Direction::Down => {
                        children.push(draw_circle_segment(i, layer, layer_size, cell_size, radius))
                    }
                    Direction::Left => children.push(draw_circle_connector(
                        i, layer, layer_size, cell_size, radius,
                    )),
                    _ => continue,
                }
            }

            if layer == radius - 1 && index != ending_index {
                children.push(draw_circle_segment(
                    i,
                    layer + 1,
                    layer_size,
                    cell_size,
                    radius,
                ));
            }

            index += 1;
        }
    }

    format!(
        "<svg width=\"{}\" height=\"{}\">{}</svg>",
        cell_size * (radius as u32 * 2),
        cell_size * (radius as u32 * 2),
        children.join("\n")
    )
}

fn draw_circle_segment(
    index: usize,
    layer: usize,
    layer_size: usize,
    cell_size: u32,
    radius: usize,
) -> String {
    let cx = (radius * cell_size as usize) as f32;
    let cy = (radius * cell_size as usize) as f32;

    let r = (layer * cell_size as usize) as f32;

    let start_angle = (index as f32 / layer_size as f32) * 2.0 * std::f32::consts::PI;
    let sweep_angle = (1.0 / layer_size as f32) * 2.0 * std::f32::consts::PI;

    let path = svg_ellipse_arc((cx, cy), (r, r), (start_angle, sweep_angle), 0.0);

    draw_path(None, Some("#000000"), path)
}

fn matrix_times(((a, b), (c, d)): ((f32, f32), (f32, f32)), (x, y): (f32, f32)) -> (f32, f32) {
    (a * x + b * y, c * x + d * y)
}

fn rotate_matrix(x: f32) -> ((f32, f32), (f32, f32)) {
    ((x.cos(), -x.sin()), (x.sin(), x.cos()))
}

fn vec_add((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> (f32, f32) {
    (x1 + x2, y1 + y2)
}

/// Only God knows how this works
fn svg_ellipse_arc(
    (cx, cy): (f32, f32),
    (rx, ry): (f32, f32),
    (start_angle, sweep_angle): (f32, f32),
    phi: f32,
) -> Vec<String> {
    let sweep_angle = sweep_angle % (2.0 * std::f32::consts::PI);
    let rot_matrix = rotate_matrix(phi);

    let (sx, sy) = vec_add(
        matrix_times(rot_matrix, (rx * start_angle.cos(), ry * start_angle.sin())),
        (cx, cy),
    );
    let (ex, ey) = vec_add(
        matrix_times(
            rot_matrix,
            (
                rx * (start_angle + sweep_angle).cos(),
                ry * (start_angle + sweep_angle).sin(),
            ),
        ),
        (cx, cy),
    );
    let fa = if sweep_angle > std::f32::consts::PI {
        1
    } else {
        0
    };
    let fs = if sweep_angle > 0.0 { 1 } else { 0 };

    let mut path = Vec::new();

    path.push(format!("M {} {}", sx, sy));
    path.push(format!(
        "A {} {} {} {} {} {} {}",
        rx,
        ry,
        phi / (2.0 * std::f32::consts::PI) * 360.0,
        fa,
        fs,
        ex,
        ey
    ));

    path
}

fn draw_circle_connector(
    index: usize,
    layer: usize,
    layer_size: usize,
    cell_size: u32,
    radius: usize,
) -> String {
    let start_x = (layer * cell_size as usize) as f32
        * ((index as f32 / layer_size as f32) * 2.0 * std::f32::consts::PI).cos()
        + (radius * cell_size as usize) as f32;

    let start_y = (layer * cell_size as usize) as f32
        * ((index as f32 / layer_size as f32) * 2.0 * std::f32::consts::PI).sin()
        + (radius * cell_size as usize) as f32;

    let end_x = ((layer + 1) * cell_size as usize) as f32
        * ((index as f32 / layer_size as f32) * 2.0 * std::f32::consts::PI).cos()
        + (radius * cell_size as usize) as f32;

    let end_y = ((layer + 1) * cell_size as usize) as f32
        * ((index as f32 / layer_size as f32) * 2.0 * std::f32::consts::PI).sin()
        + (radius * cell_size as usize) as f32;

    let mut path = Vec::new();

    path.push(format!("M {} {}", start_x, start_y));
    path.push(format!("L {} {}", end_x, end_y));

    draw_path(None, Some("#000000"), path)
}
