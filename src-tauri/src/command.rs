use lazy_static::lazy_static;
use crate::program::*;
use std::sync::Mutex;
use crate::friend::*;
use crate::graph::*;
use std::sync::Arc;


lazy_static! {
  static ref GLOBAL_GRAPH:  Arc<Mutex<Graph<Friend>>> = Arc::new(Mutex::new(Graph::new()));
}

#[tauri::command]
pub fn open_graph(path: &str) -> bool {
    // call the constructor..
    println!("Opening graph from: {}", path);
    match graph_from_file(path) {
      Ok(graph) => {
          let mut global_var = GLOBAL_GRAPH.lock().unwrap();
          *global_var = graph;
          println!("CONVERSION DONE!\n {}", *global_var);
          true
      }
      Err(error) => {
          println!("Error: {}", error);
          false
      }
  }
}

#[tauri::command]
pub fn send_graph_data() -> String{
  let mut global_var = GLOBAL_GRAPH.lock().unwrap();

  let serialized_data = serde_json::to_string(&*global_var);
  return serialized_data.unwrap().to_string();
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