// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, Menu, Submenu};
use crate::command::*;
mod command;
mod program;
mod friend;
mod graph;

fn main() {
    let open = tauri::CustomMenuItem::new("open".to_string(), "Open");
    let save: CustomMenuItem = tauri::CustomMenuItem::new("saveAs".to_string(), "SaveAs");
    let file_submenu = Submenu::new("Files", Menu::new().add_item(open).add_item(save));
    let menu = Menu::new()
        .add_submenu(file_submenu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_graph,
            save_graph
        ])
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
              "open" => {
                // Open file manager.. 
                event.window().emit("openfile", "").unwrap();
                println!("He pressed Open!");
              }
              "saveAs" => {
                // Open file manager.. to save it to a folder
                event.window().emit("saveasfile", "").unwrap();
                println!("He pressed Save!");
              }
              _ => {}
            }
          })
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
