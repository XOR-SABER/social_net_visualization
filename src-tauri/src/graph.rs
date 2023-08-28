pub use core::fmt;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

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

    // Sends the graph to the visualizer
    pub fn send_graph(&self) -> Vec<(String, Vec<String>)> {
        let mut retval: Vec<(String, Vec<String>)> = Vec::new();
        for key in &self.hash {
            retval.push((key.0.to_string(), key.1.list.clone().into_iter().collect()));
        }
        return retval;
    }

    pub fn get_connections(&self, from: &str) -> Vec<String> {
        if !self.hash.contains_key(from) {
            return Vec::new();
        }
        self
            .hash
            .get(from)
            .unwrap()
            .list
            .clone()
            .into_iter()
            .collect()
    }

    pub fn print_dfs(&self, from: &str) -> Vec<String> {
        let mut order_list: Vec<String> = Vec::new();
        let mut process_list: HashSet<String> = HashSet::new();
        self.dfs_rec(from, &mut order_list, &mut process_list);
        order_list
    }

    // Fuck I hate recursion...
    fn dfs_rec(&self, node: &str, list: &mut Vec<String>, hash: &mut HashSet<String>) {
        if hash.contains(node) {
            return;
        }
        hash.insert(node.to_string());
        // Processing starts here
        list.push(node.to_string());
        for str in self.get_connections(node) {
            self.dfs_rec(str.as_str(), list, hash);
        }
    }

    // This is actually the easist one
    pub fn print_bfs(&self, from: &str) -> Vec<String> {
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut process_list: HashSet<String> = HashSet::new();
        let mut visted: Vec<String> = Vec::new();
        queue.push_back(from.to_string());

        while !queue.is_empty() {
            let current: String = queue.pop_front().unwrap();
            visted.push(current.clone());
            process_list.insert(current.clone());
            for str in self.get_connections(&current) {
                if !process_list.contains(&str) {
                    process_list.insert(str.clone());
                    queue.push_back(str);
                }
            }
        }
        visted
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
