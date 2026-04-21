use anyhow::Context;
use specta_typescript::Typescript;
use tauri_template_lib::{bindings_path, specta_builder};

/// Exports Specta TypeScript bindings to the frontend.
///
/// # Returns
/// `Ok(())` when bindings are exported successfully.
///
/// # Errors
/// Returns an error if the bindings file cannot be generated or written.
fn main() -> anyhow::Result<()> {
    let builder = specta_builder();
    let output_path = bindings_path();

    builder
        .export(Typescript::default(), output_path)
        .context("Failed to export TypeScript bindings")?;

    Ok(())
}
