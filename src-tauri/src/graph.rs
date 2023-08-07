use core::fmt;
use std::collections::{BTreeSet, HashMap, HashSet};

use serde_derive::{Deserialize, Serialize};


// Adj list Struct "Node"
#[derive(Serialize, Deserialize)]
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

//Cool way todo operator overloading
impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str: String = String::new();
        for s in &self.list {
            str += &format!("\n\t{}", s);
        }
        return write!(format, "{}\nConnections: {}", self.data, str);
    }
}

// Graph struct
#[derive(Serialize, Deserialize)]
pub struct Graph<T> {
    hash: HashMap<String, Node<T>>,
}

// Implements the graph functions
impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            hash: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, from: &str, to: &str) -> bool {
        if !self.hash.contains_key(from) {
            return false;
        }
        self.hash.get_mut(from).unwrap().list.insert(to.to_string());
        true
    }

    pub fn remove_connection(&mut self, from: &str, to: &str) -> bool {
        if !self.hash.contains_key(from) || !self.hash.contains_key(to) {
            return false;
        }
        self.hash.get_mut(from).unwrap().list.remove(to);
        true
    }

    pub fn add_node(&mut self, to_add: Node<T>) -> bool {
        if self.hash.contains_key(&to_add.key) {
            return false;
        }
        self.hash.insert(to_add.key.clone(), to_add);
        true
    }

    pub fn remove_node(&mut self, key: &str) -> bool {
        if !self.hash.contains_key(key) {
            return false;
        }
        let node = self.hash.remove(key).unwrap();
        for conn in node.list {
            self.remove_connection(key, &conn);
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


//Cool way todo operator overloading
impl<T: fmt::Display> fmt::Display for Graph<T> {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str: String = String::new();
        for p in &self.hash {
            str += &format!("{}\n", p.1);
        }
        return write!(format, "{}", str);
    }
}