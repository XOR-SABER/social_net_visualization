use lazy_static::lazy_static;
use crate::program::*;
use std::sync::Mutex;
use crate::friend::*;
use crate::graph::*;
use std::sync::Arc;


lazy_static! {
  static ref GLOBAL_GRAPH:  Arc<Mutex<Graph<Friend>>> = Arc::new(Mutex::new(Graph::new()));
}

// Builds the graph in backend.. 
#[tauri::command]
pub fn open_graph(path: &str) -> bool {
    // call the constructor..
    println!("Opening graph from: {}", path);
    match graph_from_file(path) {
      Ok(graph) => {
          let mut global_var = GLOBAL_GRAPH.lock().unwrap();
          *global_var = graph;
          true
      }
      Err(error) => {
          println!("Error: {}", error);
          false
      }
  }
}


//Sends the graph to the frontend.. 
#[tauri::command]
pub fn send_graph_nodes() -> Vec<(String, Vec<String>)> {
  // Grab a refrence to the graph
  let binding = GLOBAL_GRAPH.lock();
  let graph_ref = binding.as_ref().unwrap();
  return graph_ref.send_graph();
}
  

// Sends the connections from a node into the frontend.. 
#[tauri::command]
pub fn send_graph_connections(id: &str) -> Vec<String> {
  let binding = GLOBAL_GRAPH.lock();
  let graph_ref = binding.as_ref().unwrap();
  return graph_ref.get_connections(id);
}




// this is going to be done last.. 
#[tauri::command]
pub fn save_graph(path : &str) -> bool {
  // call the serialze function..
  // give it a name.. 
  // write to file.. 
  println!("{}", path);
  return true;
}