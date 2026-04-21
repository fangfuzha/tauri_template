use std::path::PathBuf;

use anyhow::Context;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

mod logging;

use logging::{init_logging, log_message};

/// Returns the absolute path to the generated frontend bindings.
///
/// # Returns
/// The resolved [`PathBuf`] pointing to `src/bindings.ts`.
pub fn bindings_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/bindings.ts")
}

/// Builds a [`Builder`] with all exported Tauri commands.
///
/// # Returns
/// A configured [`Builder`] ready for export and invoke handler wiring.
pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![greet, log_message])
}

/// Greets the caller by name.
///
/// # Arguments
/// * `name` - The name to greet.
///
/// # Returns
/// A greeting message.
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Starts the Tauri application and wires up commands and bindings.
///
/// # Returns
/// No return value.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), bindings_path())
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            { init_logging((&app.handle().clone()).clone()) }
                .context("failed to initialize logging")?;
            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
