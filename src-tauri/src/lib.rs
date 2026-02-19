//! # Zenite Core
//!
//! Core Rust module for the Zenite Tauri application.
//!
//! This module contains all Tauri commands and application logic
//! that runs in the native backend.

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
