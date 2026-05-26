use std::path::PathBuf;

/// Returns the absolute path to the generated frontend bindings.
///
/// Kept as a separate utility so other tooling (like a small export binary)
/// can reuse the same resolution logic.
pub fn bindings_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/generated/bindings.ts")
}

pub mod logging;
pub mod time;
