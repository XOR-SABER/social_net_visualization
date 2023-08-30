use std::io::{BufReader, Read};
use crate::friend::*;
use crate::graph::*;
use std::fs::File;

pub fn graph_from_file(path: &str) -> Result<Graph<Friend>, String> {
    let mut new_graph: Graph<Friend> = Graph::new();
    let file = File::open(path).map_err(|err| err.to_string())?;

    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).map_err(|err| err.to_string())?;

    let lines: Vec<String> = buffer
        .lines()
        .map(|s| s.replace("\r", "")) // Remove carriage returns
        .collect();


    // Check if the file contains the signature.
    if lines.is_empty() || lines[0].trim() != "AB_VISUALIZER" {
        return Err("INVALID FILE! NOT RIGHT TYPE!".to_string());
    }

    // Create the graph
    for line in &lines[1..] {
        if line.is_empty() {
            continue; // Skip empty lines (nodes without connections).
        }

        let split_list: Vec<String> = line.split(" : ").map(String::from).collect();
        let new_node: Node<Friend> = Node::new(split_list[0].clone(), Friend::new(split_list[0].clone()));
        new_graph.add_node(new_node).unwrap();

        if split_list.len() > 1 {
            let partition: Vec<String> = split_list[1].split(" ").map(String::from).collect();
            for part in partition {
                if part.is_empty() {
                    continue;
                }
                new_graph.add_connection(&split_list[0], &part).unwrap();
            }
        }
    }

    Ok(new_graph)
}
