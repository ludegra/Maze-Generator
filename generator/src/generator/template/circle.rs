use crate::node::{Node, Connection, Direction};

pub fn generate(size: &[u32]) -> (Vec<Node>, usize, usize) {
    
    let mut size_iter = size.iter();

    // Extract values from size array
    let radius = *size_iter.next().unwrap() as usize;
    let intensity = *size_iter.next().unwrap() as usize;

    // Initialize vector for nodes
    let mut nodes = Vec::new();

    // Initialize center node
    let mut center = Node::new();

    for i in 0..nodes_in_layer(1, intensity as usize) {
        connect(&mut center, Direction::Up, i + 1);
    }

    nodes.push(center);

    let mut index = 1;
    for layer in 1..radius {
        let layer_size = nodes_in_layer(layer, intensity);
        let next_layer_size = nodes_in_layer(layer + 1, intensity);
        let previos_layer_size = nodes_in_layer(layer - 1, intensity);

        for i in 0..layer_size {
            let mut node = Node::new();

            connect_node!(node, true, Direction::Left, index + (i + layer_size - 1) % layer_size);
            connect_node!(node, true, Direction::Right, index + (i + 1) % layer_size);
            connect_node!(node, layer != 1, Direction::Down, index - previos_layer_size + if layer == previos_layer_size { i } else { i / 2 });
            connect_node!(node, layer == 1, Direction::Down, 0);

            if layer_size == next_layer_size {
                connect_node!(node, layer != radius - 1, Direction::Up, index + layer_size + i);
            }
            else {
                connect_node!(node, layer != radius - 1, Direction::Up, index + layer_size + i * 2);
                connect_node!(node, layer != radius - 1, Direction::Up, index + layer_size + i * 2 + 1);
            }

            nodes.push(node);
        }
        index += layer_size;
    }

    (nodes, 0, index - 1)
}

fn nodes_in_layer(layer: usize, intensity: usize) -> usize {
    if layer == 0 {
        1
    }
    else {
        intensity * ((layer + 1) / 2)
    } 
}

fn connect(node: &mut Node, direction: Direction, index: usize) {
    let connection = Connection {
        direction,
        index,
        active: true,
    };

    node.connect(connection);
}
