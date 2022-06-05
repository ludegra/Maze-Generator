#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Connection {
    pub direction: Direction,
    pub index: usize,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct Node {
    connections: Vec<Connection>,
}

impl Node {
    pub fn new() -> Self {
        let connections = Vec::new();

        Self { connections }
    }

    /// Adds an id of a Node to the connections
    pub fn connect(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    /// Adds multiple ids of Nodes to the connections
    pub fn connect_many(&mut self, connections: Vec<Connection>) {
        self.connections.extend(connections);
    }

    /// Removes an id of a Node from the connections
    pub fn disconnect(&mut self, index: usize) {
        self.connections.retain(|&x| x.index != index);
    }

    /// Removes multiple ids of Nodes from the connections
    pub fn disconnect_multiple(&mut self, ids: &[usize]) {
        self.connections.retain(|&x| !ids.contains(&x.index));
    }

    /// Returns the connections of the Node   
    pub fn get_connections(&self) -> Vec<Connection> {
        self.connections.clone()
    }

    /// Returns a mutable reference to the connections of the Node
    pub fn connections_mut(&mut self) -> &mut Vec<Connection> {
        &mut self.connections
    }

    pub fn disable_connection(&mut self, index: usize) {
        self.connections.iter_mut().find(|x| x.index == index).unwrap().active = false;
    }
}