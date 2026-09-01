#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
    priority: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoData {
    todos: Vec<TodoItem>,
    next_id: u32,
}

fn get_data_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    path.push("TodoApp");
    path.push("todos.json");
    path
}

fn ensure_dir(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
}

#[tauri::command]
fn load_todos() -> Result<TodoData, String> {
    let path = get_data_path();
    if !path.exists() {
        return Ok(TodoData { todos: vec![], next_id: 1 });
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let data: TodoData = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(data)
}

#[tauri::command]
fn save_todos(data: TodoData) -> Result<(), String> {
    let path = get_data_path();
    ensure_dir(&path);
    let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![load_todos, save_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
