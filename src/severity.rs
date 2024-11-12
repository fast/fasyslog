// Copyright 2024 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::str::FromStr;

/// Syslog severity as defined in [RFC 5424] (The Syslog Protocol).
///
/// [RFC 5424]: https://datatracker.ietf.org/doc/html/rfc5424.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Severity {
    /// Emergency: system is unusable, numerical code 0.
    EMERGENCY = 0,
    /// Alert: action must be taken immediately, numerical code 1.
    ALERT = 1,
    /// Critical: critical conditions, numerical code 2.
    CRITICAL = 2,
    /// Error: error conditions, numerical code 3.
    ERROR = 3,
    /// Warning: warning conditions, numerical code 4.
    WARNING = 4,
    /// Notice: normal but significant condition, numerical code 5.
    NOTICE = 5,
    /// Informational: informational messages, numerical code 6.
    INFORMATIONAL = 6,
    /// Debug: debug-level messages, numerical code 7.
    DEBUG = 7,
}

impl Severity {
    /// Returns the numerical code of the severity.
    pub fn code(self) -> u8 {
        self as u8
    }

    /// Returns the label of the severity.
    pub fn label(self) -> &'static str {
        match self {
            Severity::EMERGENCY => "EMERGENCY",
            Severity::ALERT => "ALERT",
            Severity::CRITICAL => "CRITICAL",
            Severity::ERROR => "ERROR",
            Severity::WARNING => "WARNING",
            Severity::NOTICE => "NOTICE",
            Severity::INFORMATIONAL => "INFORMATIONAL",
            Severity::DEBUG => "DEBUG",
        }
    }
}

impl TryFrom<u8> for Severity {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Severity::EMERGENCY),
            1 => Ok(Severity::ALERT),
            2 => Ok(Severity::CRITICAL),
            3 => Ok(Severity::ERROR),
            4 => Ok(Severity::WARNING),
            5 => Ok(Severity::NOTICE),
            6 => Ok(Severity::INFORMATIONAL),
            7 => Ok(Severity::DEBUG),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Severity {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &value.to_lowercase()[..] {
            "emergency" => Ok(Severity::EMERGENCY),
            "alert" => Ok(Severity::ALERT),
            "critical" => Ok(Severity::CRITICAL),
            "error" => Ok(Severity::ERROR),
            "warning" => Ok(Severity::WARNING),
            "notice" => Ok(Severity::NOTICE),
            "informational" => Ok(Severity::INFORMATIONAL),
            "debug" => Ok(Severity::DEBUG),
            _ => Err(()),
        }
    }
}

impl FromStr for Severity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}
