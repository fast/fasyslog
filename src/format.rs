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

//! Format Syslog messages according to the referred standards.

use std::fmt;
use std::fmt::Formatter;

use jiff::Timestamp;
use jiff::Zoned;

use crate::internal::hostname;
use crate::Facility;
use crate::SDElement;
use crate::Severity;

const NILVALUE: &str = "-";

/// Shared context for constructing Syslog messages.
#[derive(Debug, Clone)]
pub struct SyslogContext {
    facility: Facility,
    hostname: Option<String>,
    appname: Option<String>,
    procid: Option<String>,
}

impl Default for SyslogContext {
    fn default() -> Self {
        Self::new()
    }
}

impl SyslogContext {
    /// Create a new blank `SyslogContext`.
    pub const fn const_new() -> Self {
        Self {
            facility: Facility::USER,
            hostname: None,
            appname: None,
            procid: None,
        }
    }

    /// Create a new `SyslogContext` with system default values.
    pub fn new() -> Self {
        let procid = std::process::id();
        let appname = std::env::current_exe().ok().and_then(|path| {
            // NOTE - cannot chain because 'file_name'/'to_str' return a temporary reference
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_string())
        });
        let hostname = hostname().and_then(|name| name.to_str().map(|name| name.to_string()));
        Self {
            facility: Facility::USER,
            hostname,
            appname,
            procid: Some(procid.to_string()),
        }
    }

    /// Set the facility of the Syslog message.
    pub fn facility(&mut self, facility: Facility) -> &mut Self {
        self.facility = facility;
        self
    }

    /// Set the hostname of the Syslog message.
    pub fn hostname(&mut self, hostname: impl Into<String>) -> &mut Self {
        self.hostname = Some(hostname.into());
        self
    }

    /// Set the appname of the Syslog message.
    pub fn appname(&mut self, appname: impl Into<String>) -> &mut Self {
        self.appname = Some(appname.into());
        self
    }

    /// Set the procid of the Syslog message.
    pub fn procid(&mut self, procid: impl Into<String>) -> &mut Self {
        self.procid = Some(procid.into());
        self
    }

    /// Format the Syslog message with the given severity as defined in RFC-3164.
    pub fn format_rfc3164<M>(&self, severity: Severity, message: Option<M>) -> RFC3164Formatter<M> {
        RFC3164Formatter {
            context: self,
            severity,
            message,
        }
    }

    /// Format the Syslog message with the given severity as defined in RFC-5424.
    pub fn format_rfc5424<S, M>(
        &self,
        severity: Severity,
        msgid: Option<S>,
        elements: Vec<SDElement>,
        message: Option<M>,
    ) -> RFC5424Formatter<M>
    where
        S: Into<String>,
        M: fmt::Display,
    {
        let msgid = msgid.map(|s| s.into());
        RFC5424Formatter {
            context: self,
            severity,
            msgid,
            elements,
            message,
        }
    }
}

/// Shared format logic for nullable value.
fn nullable_value(value: Option<&str>) -> &str {
    value.unwrap_or(NILVALUE)
}

/// Format the Syslog message as [RFC-3164] (BSD syslog Protocol).
///
/// [RFC-3164]: https://datatracker.ietf.org/doc/html/rfc3164
#[derive(Debug)]
pub struct RFC3164Formatter<'a, M> {
    context: &'a SyslogContext,
    severity: Severity,
    message: Option<M>,
}

impl<M> fmt::Display for RFC3164Formatter<'_, M>
where
    M: fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // PRI (priority) Part
        // https://datatracker.ietf.org/doc/html/rfc3164#section-4.1.1
        let pri = (self.context.facility.code() << 3) | self.severity.code();
        // HEADER Part of a syslog Packet
        // https://datatracker.ietf.org/doc/html/rfc3164#section-4.1.2
        let ts = Zoned::now().strftime("%b %e %T");
        let hostname = nullable_value(self.context.hostname.as_deref());
        let appname = nullable_value(self.context.appname.as_deref());
        write!(f, "<{pri}>{ts} {hostname} {appname}")?;
        // Conventions defined in RFC-3164 §5.3
        // At least, this is the behavior of Ubuntu 24.04 LTS.
        if let Some(procid) = &self.context.procid {
            write!(f, "[{procid}]")?;
        }
        if let Some(message) = &self.message {
            write!(f, ": {}", message)?;
        }
        Ok(())
    }
}

/// Format the Syslog message as [RFC 5424] (The Syslog Protocol)
///
/// [RFC 5424]: https://datatracker.ietf.org/doc/html/rfc5424
#[derive(Debug)]
pub struct RFC5424Formatter<'a, M> {
    context: &'a SyslogContext,
    severity: Severity,
    msgid: Option<String>,
    elements: Vec<SDElement>,
    message: Option<M>,
}

impl<M> fmt::Display for RFC5424Formatter<'_, M>
where
    M: fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // The PRI (priority) part is defined in RFC-5424 §6.2.1.
        // https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1
        let pri = (self.context.facility.code() << 3) | self.severity.code();
        // The VERSION field denotes the version of the syslog protocol specification.
        // https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.2
        let ver = 1;
        // Jiff ensures that Timestamp is always displayed as an RFC-3339 compliant string.
        // https://docs.rs/jiff/*/jiff/struct.Timestamp.html#impl-Display-for-Timestamp
        let ts = Timestamp::now();
        let hostname = nullable_value(self.context.hostname.as_deref());
        let appname = nullable_value(self.context.appname.as_deref());
        let procid = nullable_value(self.context.procid.as_deref());
        let msgid = nullable_value(self.msgid.as_deref());
        write!(
            f,
            "<{pri}>{ver} {ts:.6} {hostname} {appname} {procid} {msgid} "
        )?;
        if self.elements.is_empty() {
            write!(f, "-")?;
        } else {
            for element in &self.elements {
                write!(f, "{element}")?;
            }
        }
        if let Some(message) = &self.message {
            write!(f, " {}", message)?;
        }
        Ok(())
    }
}
