use std::time::{SystemTime, UNIX_EPOCH};

/// Returns current timestamp in milliseconds since UNIX_EPOCH.
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
