use std::sync::atomic::{AtomicU32, Ordering};

use wasm_bindgen::prelude::wasm_bindgen;

static ID: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
pub struct Node {
    _id: u32,
    connections: Vec<u32>
}

impl Node {
    pub fn new() -> Self {
        let id = ID.fetch_add(1, Ordering::Relaxed);
        let connections = Vec::new();

        Self { _id: id, connections }
    }

    /// Adds an id of a Node to the connections
    pub fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }

    /// Adds multiple ids of Nodes to the connections
    pub fn connect_multiple(&mut self, ids: &[u32]) {
        self.connections.extend(ids);
    }

    /// Removes an id of a Node from the connections
    pub fn disconnect(&mut self, id: u32) {
        self.connections.retain(|&x| x != id);
    }

    /// Removes multiple ids of Nodes from the connections
    pub fn disconnect_multiple(&mut self, ids: &[u32]) {
        self.connections.retain(|&x| !ids.contains(&x));
    }
}

#[wasm_bindgen]
impl Node {
    pub fn get_connections(&self) -> Vec<u32> {
        self.connections.clone()
    }
}