use crate::node::Node;

/// Generates a square grid with the width and height read from the first two elements of the size slice.
/// 
/// ## Panics
/// 
/// Panics if the size slice is shorter than two elements.
pub fn generate(size: &[u32]) -> Vec<Node> {
    let mut grid = Vec::new();

    assert!(size.len() >= 2);

    let width = size[0];
    let length = size[1];

    for y in 0..length {
        for x in 0..width {
            let mut node = Node::new();

            let x_max = width - 1;
            let y_max = length - 1;

            // Adds neighbors to the node
            // Check if node is on the edge of the grid

            // X-axis
            match x {
                0 => node.connect(x + 1),
                k if (k == x_max) => node.connect(x - 1),
                _ => node.connect_multiple(&[x - 1, x + 1]),
            }

            // Y-axis
            match y {
                0 => node.connect(y + 1),
                k if (k == y_max) => node.connect(y - 1),
                _ => node.connect_multiple(&[y - 1, y + 1]),
            }

            grid.push(node);
        }
    }

    grid
}