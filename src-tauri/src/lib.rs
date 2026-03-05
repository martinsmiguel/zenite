//! # Zenite Core
//!
//! Core Rust module for the Zenite Tauri application.
//!
//! This module contains all Tauri commands and application logic
//! that runs in the native backend.

pub mod db;
pub mod models;

use db::ensure_database_exists;
use models::DatabaseConfig;

/// Returns a personalized greeting message.
///
/// # Arguments
///
/// * `name` - The name of the person to greet. Must be a non-empty string.
///
/// # Returns
///
/// A formatted greeting string including the provided name.
///
/// # Example
///
/// ```
/// let message = greet("World");
/// assert_eq!(message, "Hello, World! You've been greeted from Rust!");
/// ```
///
/// # TypeScript Contract
///
/// Input: `{ name: string }`
/// Output: `string`
///
/// See: `docs/architecture/tauri-contract-api.md`
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init_database(path: &str) -> Result<String, String> {
    let config = DatabaseConfig {
        path: path.to_string(),
    };
    let _conn = ensure_database_exists(&config)
        .map_err(|e| format!("Erro ao inicializar database: {}", e))?;
    Ok("Database inicializado com sucesso".to_string())
}

#[tauri::command]
fn database_exists(path: &str) -> Result<bool, String> {
    Ok(std::path::Path::new(path).exists())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            init_database,
            database_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
