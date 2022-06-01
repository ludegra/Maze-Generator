use rand::prelude::SliceRandom;

use crate::node::Node;

pub fn generate(maze: &mut Vec<Node>, starting_index: usize, ending_index: usize) {
    search(maze, &mut Vec::new(), starting_index, ending_index);
}

/// Uses depth-first search to generate a random maze
fn search(nodes: &mut Vec<Node>, visited: &mut Vec<usize>, index: usize, goal: usize) {
    let node = &mut nodes[index];
    visited.push(index);

    let mut connections = node.get_connections();
    connections.shuffle(&mut rand::thread_rng());

    if index != goal {
        for connection in connections.iter_mut() {
            if !visited.contains(&connection.index) {
                connection.active = false;
                nodes[connection.index].disable_connection(index);

                search(nodes, visited, connection.index, goal);
            }
        }
    }
}
