use std::fmt;

use crate::bencher::sub::RunError;

const UNIX_FLAG: &str = "-c";
const WINDOWS_FLAG: &str = "/C";

#[derive(Debug, Clone)]
pub enum Flag {
    Unix,
    Windows,
    Custom(String),
}

impl TryFrom<Option<String>> for Flag {
    type Error = RunError;

    fn try_from(shell: Option<String>) -> Result<Self, Self::Error> {
        Ok(if let Some(shell) = shell {
            Self::Custom(shell)
        } else if cfg!(target_family = "unix") {
            Self::Unix
        } else if cfg!(target_family = "windows") {
            Self::Windows
        } else {
            return Err(RunError::Flag);
        })
    }
}

impl AsRef<str> for Flag {
    fn as_ref(&self) -> &str {
        match self {
            Self::Unix => UNIX_FLAG,
            Self::Windows => WINDOWS_FLAG,
            Self::Custom(shell) => shell,
        }
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
