#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonLogging {
    pub name: String,
    pub log: ServerLog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
#[expect(variant_size_differences)]
pub enum ServerLog {
    StderrTerminal {
        level: LogLevel,
    },
    File {
        level: LogLevel,
        path: String,
        if_exists: IfExists,
    },
}

impl ServerLog {
    pub fn level(&self) -> LogLevel {
        match self {
            Self::StderrTerminal { level } | Self::File { level, .. } => level.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum IfExists {
    Fail,
    Truncate,
    Append,
}
