use anyhow::Context;
use specta_typescript::Typescript;

mod command;
mod utils;

use utils::logging::init_logging;

pub use command::specta_builder;
pub use utils::bindings_path;

/// Starts the Tauri application and wires up commands and bindings.
///
/// # Returns
/// No return value.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();
    let max_level = if cfg!(debug_assertions) {
        "trace"
    } else {
        "info"
    }
    .to_string();

    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), bindings_path())
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            { init_logging((&app.handle().clone()).clone(), max_level.clone()) }
                .context("failed to initialize logging")?;
            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
