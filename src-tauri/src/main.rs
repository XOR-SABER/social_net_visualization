// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, Menu, Submenu};
use crate::friend::Friend;
use crate::graph::Graph;
mod friend;
mod graph;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let social_graph : Graph<Friend> = Graph::new();
    let open = tauri::CustomMenuItem::new("open".to_string(), "Open");
    let save: CustomMenuItem = tauri::CustomMenuItem::new("save".to_string(), "Save");
    let file_submenu = Submenu::new("Files", Menu::new().add_item(open).add_item(save));
    let menu = Menu::new()
        .add_submenu(file_submenu);
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
              "open" => {
                // Open file manager.. 
                event.window().emit("openfile", "").unwrap();
                println!("He pressed Open!");
              }
              "save" => {
                // Open file manager.. to save it to a folder
                event.window().emit("savefile", "").unwrap();
                println!("He pressed Save!");
              }
              _ => {}
            }
          })
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
