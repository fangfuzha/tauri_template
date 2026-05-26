use tauri_specta::{collect_commands, Builder};

use crate::utils::logging;

/// Greets the caller by name.
///
/// This handler is kept next to command builder so command exports and
/// collection live together.
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Builds a [`Builder`] with all exported Tauri commands.
///
/// Returns a configured [`Builder`] ready for export and invoke handler wiring.
pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        greet,
        logging::log_message,
        logging::begin_log_mirror,
        logging::end_log_mirror
    ])
}
