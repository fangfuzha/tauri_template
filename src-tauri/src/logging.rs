use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::Emitter;

pub const LOG_EVENT: &str = "tauri_template:log";

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
static LOGGER: BridgeLogger = BridgeLogger;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<log::Level> for LogLevel {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Trace => Self::Trace,
            log::Level::Debug => Self::Debug,
            log::Level::Info => Self::Info,
            log::Level::Warn => Self::Warn,
            log::Level::Error => Self::Error,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Type, Eq, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogSource {
    #[default]
    Frontend,
    Backend,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogContextItem {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogMessageInput {
    pub level: LogLevel,
    pub message: String,

    #[serde(default)]
    pub source: LogSource,

    #[serde(default)]
    pub target: Option<String>,

    #[serde(default)]
    pub context: Vec<LogContextItem>,

    #[serde(default)]
    pub file: Option<String>,

    #[serde(default)]
    pub line: Option<u32>,

    #[serde(default)]
    pub module_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub timestamp_ms: u64,
    pub level: LogLevel,
    pub source: LogSource,
    pub message: String,
    pub target: Option<String>,
    pub context: Vec<LogContextItem>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub module_path: Option<String>,
}

impl LogMessageInput {
    fn into_entry(self) -> LogEntry {
        LogEntry {
            timestamp_ms: current_timestamp_ms(),
            level: self.level,
            source: self.source,
            message: self.message,
            target: self.target,
            context: self.context,
            file: self.file,
            line: self.line,
            module_path: self.module_path,
        }
    }
}

/// Installs the backend logger and stores the app handle for log forwarding.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle used to broadcast log events.
///
/// # Errors
/// Returns an error if the global logger cannot be initialized.
pub fn init_logging(app_handle: tauri::AppHandle) -> Result<()> {
    APP_HANDLE
        .set(app_handle)
        .map_err(|_| anyhow::anyhow!("logging already initialized"))?;

    log::set_logger(&LOGGER).map_err(|_| anyhow::anyhow!("failed to install logger"))?;
    log::set_max_level(log::LevelFilter::Trace);

    Ok(())
}

/// Publishes a structured log entry to the terminal and frontend event channel.
///
/// # Arguments
/// * `entry` - The log entry to publish.
pub fn publish_log(entry: LogEntry) {
    let rendered = render_log_entry(&entry);

    match entry.level {
        LogLevel::Warn | LogLevel::Error => eprintln!("{rendered}"),
        _ => println!("{rendered}"),
    }

    if let Some(app_handle) = APP_HANDLE.get() {
        if let Err(error) = app_handle.emit(LOG_EVENT, &entry) {
            eprintln!("[log bridge] failed to emit frontend log event: {error}");
        }
    }
}

/// Receives a structured log message from the frontend and rebroadcasts it.
///
/// # Arguments
/// * `input` - The structured frontend log payload.
///
/// # Returns
/// The enriched log entry that was dispatched.
#[tauri::command]
#[specta::specta]
pub fn log_message(input: LogMessageInput) -> LogEntry {
    let entry = input.into_entry();
    publish_log(entry.clone());
    entry
}

struct BridgeLogger;

impl log::Log for BridgeLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        publish_log(LogEntry {
            timestamp_ms: current_timestamp_ms(),
            level: record.level().into(),
            source: LogSource::Backend,
            message: record.args().to_string(),
            target: Some(record.target().to_string()),
            context: Vec::new(),
            file: record.file().map(ToOwned::to_owned),
            line: record.line(),
            module_path: record.module_path().map(ToOwned::to_owned),
        });
    }

    fn flush(&self) {}
}

fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn render_log_entry(entry: &LogEntry) -> String {
    let mut rendered = format!(
        "[{level}][{source}][{target}] {message}",
        level = format_log_level(entry.level),
        source = format_log_source(entry.source),
        target = entry.target.as_deref().unwrap_or("app"),
        message = entry.message,
    );

    if let Some(module_path) = &entry.module_path {
        rendered.push_str(&format!(" [{module_path}]"));
    }

    if let Some(file) = &entry.file {
        if let Some(line) = entry.line {
            rendered.push_str(&format!(" ({file}:{line})"));
        } else {
            rendered.push_str(&format!(" ({file})"));
        }
    }

    if !entry.context.is_empty() {
        let context = entry
            .context
            .iter()
            .map(|item| format!("{}={}", item.key, item.value))
            .collect::<Vec<_>>()
            .join(", ");
        rendered.push_str(&format!(" {{{context}}}"));
    }

    rendered
}

fn format_log_level(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Trace => "trace",
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warn => "warn",
        LogLevel::Error => "error",
    }
}

fn format_log_source(source: LogSource) -> &'static str {
    match source {
        LogSource::Frontend => "frontend",
        LogSource::Backend => "backend",
    }
}
