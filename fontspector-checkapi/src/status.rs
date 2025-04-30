use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::Override;
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[serde(rename_all = "UPPERCASE")]
/// A severity level for a single check subresult
pub enum StatusCode {
    /// Skip: the check didn't run because some condition was not met
    Skip,
    /// Pass: there's no problem here
    Pass,
    /// Info: the check returned some useful information, but no problems
    Info,
    /// Warn: a problem which should be manually reviewed
    Warn,
    /// Fail: a problem materially affects the correctness of the font
    Fail,
    /// Error: something went wrong
    ///
    /// An Error is when something which returns a `Result<>` gave us
    /// an `Err`` - for example a file couldn't be found or couldn't be
    /// parsed, even though we did our best to check for things. In
    /// other words, it's something so bad there's no point continuing
    /// with the check; it's equivalent to a Fontbakery FATAL.
    Error,
}

impl FromStr for StatusCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SKIP" => Ok(StatusCode::Skip),
            "INFO" => Ok(StatusCode::Info),
            "PASS" => Ok(StatusCode::Pass),
            "WARN" => Ok(StatusCode::Warn),
            "FAIL" => Ok(StatusCode::Fail),
            "ERROR" => Ok(StatusCode::Error),
            _ => Err(()),
        }
    }
}

impl StatusCode {
    /// Return an iterator over all status codes
    ///
    /// Used to provide a list of possible status codes to the user when
    /// selecting the minimum reported status.
    pub fn all() -> impl Iterator<Item = StatusCode> {
        vec![
            StatusCode::Skip,
            StatusCode::Info,
            StatusCode::Pass,
            StatusCode::Warn,
            StatusCode::Fail,
            StatusCode::Error,
        ]
        .into_iter()
    }

    /// Convert a string to a status code
    ///
    /// This is used when the status code comes from an external source,
    /// such as FontBakery.
    pub fn from_string(s: &str) -> Option<StatusCode> {
        FromStr::from_str(s).ok()
    }
}
impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StatusCode::Pass => write!(f, "PASS"),
            StatusCode::Skip => write!(f, "SKIP"),
            StatusCode::Fail => write!(f, "FAIL"),
            StatusCode::Warn => write!(f, "WARN"),
            StatusCode::Info => write!(f, "INFO"),
            StatusCode::Error => write!(f, "ERROR"),
        }
    }
}
#[derive(Debug, Clone, Serialize)]
/// A status message from a check
///
/// This is a subresult, in the sense that a check may return multiple failures
/// and warnings; all the subresults then get wrapped into a [crate::CheckResult]
/// which is the final result of the check.
pub struct Status {
    /// A message to explain the status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The severity of the status
    pub severity: StatusCode,
    /// A code to identify the status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Additional metadata provided to the reporter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**{:}**: ", self.severity)?;
        if let Some(code) = self.code.as_ref() {
            write!(f, "[{}]: ", code)?;
        }
        if let Some(message) = self.message.as_ref() {
            write!(f, "{:}", message)?;
        }
        Ok(())
    }
}

impl Status {
    /// Return a single pass result from a check
    pub fn just_one_pass() -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::pass()].into_iter())
    }

    /// Return a single warn result from a check
    pub fn just_one_warn(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::warn(code, message)].into_iter())
    }

    /// Return a single info result from a check
    pub fn just_one_info(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::info(code, message)].into_iter())
    }

    /// Return a single fail result from a check
    pub fn just_one_fail(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::fail(code, message)].into_iter())
    }

    /// Return a single skip result from a check
    pub fn just_one_skip(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::skip(code, message)].into_iter())
    }

    /// Create a status with a pass severity
    pub fn pass() -> Self {
        Self {
            message: None,
            code: None,
            severity: StatusCode::Pass,
            metadata: None,
        }
    }
    /// Create a status with a fail severity
    pub fn fail(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Fail,
            metadata: None,
        }
    }
    /// Create a status with a warning severity
    pub fn warn(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Warn,
            metadata: None,
        }
    }
    /// Create a status with an info severity
    pub fn skip(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Skip,
            metadata: None,
        }
    }
    /// Create a status with an info severity
    pub fn info(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Info,
            metadata: None,
        }
    }
    /// Create a status with an error severity
    pub fn error(code: Option<&str>, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: code.map(|x| x.to_string()),
            severity: StatusCode::Error,
            metadata: None,
        }
    }

    /// Apply an override to the status
    ///
    /// Overrides are provided by the profile or by the user's configuration file;
    /// they are used to override the severity of a check result.
    pub fn process_override(&mut self, overrides: &[Override]) {
        let code = self.code.as_deref();
        if let Some(override_) = overrides.iter().find(|x| Some(x.code.as_str()) == code) {
            self.severity = override_.status;
            self.message = Some(format!(
                "{} (Overriden: {})",
                self.message
                    .clone()
                    .unwrap_or("No original message".to_string()),
                override_.reason
            ))
        }
    }
}

/// Reflects the result of some kind of early return from a check function
///
/// This may be because there was an error, or because the check was skipped.
#[derive(Debug)]
pub enum CheckError {
    /// An error occurred
    Error(String),
    /// The check was skipped due to an error condition
    Skip {
        /// Code to identify the skip reason
        code: String,
        /// Message to explain the skip to the user
        message: String,
    },
}

impl<T> From<T> for CheckError
where
    T: std::error::Error,
{
    fn from(e: T) -> Self {
        CheckError::Error(e.to_string())
    }
}

impl CheckError {
    /// Return an error which skips the check
    ///
    /// This allows you to skip a check early if an error is raised, for example
    /// if a particular table is missing from the font.
    pub fn skip(code: &str, message: &str) -> Self {
        CheckError::Skip {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

/// A list of statuses
pub type StatusList = Box<dyn Iterator<Item = Status>>;
/// The expected return type of a check implementation function
pub type CheckFnResult = Result<StatusList, CheckError>;
