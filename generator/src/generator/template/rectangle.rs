use crate::node::{Node, Connection, Direction};

/// Generates a square grid with the width and height read from the first two elements of the size slice.
/// 
/// ## Panics
/// 
/// Panics if the size slice is shorter than two elements.
pub fn generate(size: &[u32]) -> (Vec<Node>, usize, usize) {
    let mut cells = Vec::<Node>::with_capacity((size[0] * size[1]) as usize);

    // Extract values from size array
    let mut size_iter = size.into_iter();

    let width = *size_iter.next().unwrap() as usize;
    let height = *size_iter.next().unwrap() as usize;
    let start_x = *size_iter.next().unwrap() as usize;
    let start_y = *size_iter.next().unwrap() as usize;
    let end_x = *size_iter.next().unwrap() as usize;
    let end_y = *size_iter.next().unwrap() as usize;

    let mut index = 0usize;

    for y in 0..height {
        for x in 0..width {
            let mut node = Node::new();

            macro_rules! connect {
                ($connection:expr, $direction:expr, $index:expr) => {
                    if $connection {
                        node.connect(Connection {
                            direction: $direction,
                            index: $index,
                            active: true,
                        });
                    }
                };
            }

            connect!(x > 0, Direction::Left, index - 1);
            connect!(y > 0, Direction::Up, index - width);
            connect!(x < width - 1, Direction::Right, index + 1);
            connect!(y < height - 1, Direction::Down, index + width);

            index += 1;
            cells.push(node);
        }
    }

    (cells, start_y * width + start_x, end_y * width + end_x)
}