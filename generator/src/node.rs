use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
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

    /// Returns the connections of the Node   
    pub fn get_connections(&self) -> Vec<Connection> {
        self.connections.clone()
    }

    pub fn disable_connection(&mut self, index: usize) {
        self.connections.iter_mut().find(|x| x.index == index).unwrap().active = false;
    }
}