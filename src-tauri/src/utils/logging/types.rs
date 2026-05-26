//! Centralized logging-related types for Specta/TS bindings.
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fmt;

/// Log level exposed to the frontend and bindings.
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

impl From<LogLevel> for log::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => log::Level::Trace,
            LogLevel::Debug => log::Level::Debug,
            LogLevel::Info => log::Level::Info,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Error => log::Level::Error,
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for LogSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogSource::Frontend => "frontend",
            LogSource::Backend => "backend",
        };
        write!(f, "{}", s)
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
    #[serde(default)]
    pub id: Option<String>,

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
    pub id: String,

    #[specta(type = specta_typescript::Number)]
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
