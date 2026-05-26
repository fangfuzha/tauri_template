use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::OnceLock;

use anyhow::Result;
use tauri::Emitter;

use super::types::{LogEntry, LogLevel, LogMessageInput, LogSource};
use crate::utils::time::current_timestamp_ms;

/// 发送到前端的日志消息的事件名称。
pub const LOG_EVENT: &str = "tauri_template:log";

/// 全局应用句柄，用于日志事件转发到前端。使用 `OnceLock` 确保只初始化一次。
static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
/// 全局日志记录器实例，负责捕获后端日志并转发到前端。实现 `log::Log` trait。
static LOGGER: BridgeLogger = BridgeLogger;
/// 全局递增日志 ID 生成器，确保每条日志都有唯一标识符。
static LOG_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
/// 当前前端日志镜像订阅者数量。大于 0 时才向前端发事件。
static MIRROR_SUBSCRIBERS: AtomicUsize = AtomicUsize::new(0);

/// Installs the backend logger and stores the app handle for log forwarding.
///
/// # Arguments
/// * `app_handle` - The Tauri app handle used to broadcast log events.
/// * `max_level` - The maximum log level, expressed as a string like `trace`,
///   `debug`, `info`, `warn`, or `error`.
///
/// # Errors
/// Returns an error if the global logger cannot be initialized.
pub fn init_logging(app_handle: tauri::AppHandle, max_level: String) -> Result<()> {
    let max_level = parse_level_filter(&max_level)?;

    APP_HANDLE
        .set(app_handle)
        .map_err(|_| anyhow::anyhow!("logging already initialized"))?;

    log::set_logger(&LOGGER).map_err(|_| anyhow::anyhow!("failed to install logger"))?;
    log::set_max_level(max_level);

    Ok(())
}

/// Publishes a structured log entry to the terminal and frontend event channel.
///
/// # Arguments
/// * `entry` - The log entry to publish.
pub fn publish_log(entry: LogEntry) {
    let rendered = render_log_entry(&entry);

    // Warn and error logs go to stderr, the rest to stdout.
    match entry.level {
        LogLevel::Warn | LogLevel::Error => eprintln!("{rendered}"),
        _ => println!("{rendered}"),
    }

    // Mirror to frontend only when at least one subscriber is active.
    if MIRROR_SUBSCRIBERS.load(Ordering::Relaxed) > 0 {
        if let Some(app_handle) = APP_HANDLE.get() {
            if let Err(error) = app_handle.emit(LOG_EVENT, &entry) {
                eprintln!("[log bridge] failed to emit frontend log event: {error}");
            }
        }
    }
}

/// Starts mirroring backend logs to the frontend event stream.
#[tauri::command]
#[specta::specta]
pub fn begin_log_mirror() {
    MIRROR_SUBSCRIBERS.fetch_add(1, Ordering::Relaxed);
}

/// Stops mirroring backend logs to the frontend event stream for one subscriber.
#[tauri::command]
#[specta::specta]
pub fn end_log_mirror() {
    MIRROR_SUBSCRIBERS
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            Some(current.saturating_sub(1))
        })
        .ok();
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
    let entry = LogEntry {
        id: input.id.unwrap_or_else(next_log_id),
        timestamp_ms: current_timestamp_ms(),
        level: input.level,
        source: input.source,
        message: input.message,
        target: input.target,
        context: input.context,
        file: input.file,
        line: input.line,
        module_path: input.module_path,
    };
    if is_enabled_level(entry.level) {
        publish_log(entry.clone());
    }
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
            id: next_log_id(),
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

fn render_log_entry(entry: &LogEntry) -> String {
    let mut rendered = format!(
        "[{level}][{source}][{target}] {message}",
        level = entry.level,
        source = entry.source,
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

fn next_log_id() -> String {
    let counter = LOG_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!(
        "{}-{}-{}",
        current_timestamp_ms(),
        std::process::id(),
        counter
    )
}

fn is_enabled_level(level: LogLevel) -> bool {
    let filter = log::max_level();

    if filter == log::LevelFilter::Off {
        return false;
    }

    let current: log::Level = level.into();
    current <= filter.to_level().unwrap_or(log::Level::Error)
}

fn parse_level_filter(level: &str) -> Result<log::LevelFilter> {
    match level.trim().to_ascii_lowercase().as_str() {
        "off" => Ok(log::LevelFilter::Off),
        "error" => Ok(log::LevelFilter::Error),
        "warn" | "warning" => Ok(log::LevelFilter::Warn),
        "info" => Ok(log::LevelFilter::Info),
        "debug" => Ok(log::LevelFilter::Debug),
        "trace" => Ok(log::LevelFilter::Trace),
        other => Err(anyhow::anyhow!(
            "invalid logging max level: {other}. expected one of: off, error, warn, info, debug, trace"
        )),
    }
}
