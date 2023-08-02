use std::collections::{BTreeSet, HashMap, HashSet};

// Adj list Struct "Node"
pub struct Node<T> {
    pub key: String,
    pub data: T,
    pub list: BTreeSet<String>,
}

// Implmements the node functions
impl<T> Node<T> {
    pub fn new(new_key: String, new_data: T) -> Self {
        return Node {
            key: new_key,
            data: new_data,
            list: BTreeSet::new(),
        };
    }
}

// Graph struct
pub struct Graph<T> {
    hash: HashMap<String, Node<T>>,
    pub is_dag: bool,
}

// Implmements the graph functions
impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            hash: HashMap::new(),
            is_dag: false,
        }
    }
    pub fn add_connection(&mut self, from: String, to: String) -> bool {
        if !self.hash.contains_key(&from) || !self.hash.contains_key(&to) {
            return false;
        }
        self.hash.get_mut(&from).unwrap().list.insert(to.clone());
        if !self.is_dag {
            self.hash.get_mut(&to).unwrap().list.insert(from);
        }
        true
    }
    pub fn remove_connection(&mut self, from: String, to: String) -> bool {
        if !self.hash.contains_key(&from) || !self.hash.contains_key(&to) {
            return false;
        }
        self.hash
            .get_mut(&from.clone())
            .unwrap()
            .list
            .remove(&to.clone());
        if !self.is_dag {
            self.hash.get_mut(&to).unwrap().list.remove(&from);
        }
        true
    }
    pub fn add_node(&mut self, to_add: Node<T>) -> bool {
        if self.hash.contains_key(&to_add.key) {
            return false;
        }
        self.hash.insert(to_add.key.clone(), to_add);
        true
    }
    pub fn remove_node(&mut self, key: String) -> bool {
        if !self.hash.contains_key(&key) {
            return false;
        }
        let node = self.hash.remove(&key).unwrap();
        for conn in node.list {
            self.remove_connection(key.clone(), conn.clone());
        }
        true
    }
    pub fn print_network(&self, from: &str) -> HashSet<String> {
        let mut processed = HashSet::new();
        let mut to_process = vec![from.to_string()];
        while !to_process.is_empty() {
            let current = to_process.pop().unwrap();
            if !processed.contains(&current) {
                processed.insert(current.clone());
                if let Some(node) = self.hash.get(&current) {
                    for conn in &node.list {
                        to_process.push(conn.clone());
                    }
                }
            }
        }
        processed
    }
}