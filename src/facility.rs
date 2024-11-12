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

/// Syslog facility as defined in [RFC 5424] (The Syslog Protocol).
///
/// See also [RFC 5427] (Textual Conventions for Syslog Management) for the labels.
///
/// [RFC 5424]: https://datatracker.ietf.org/doc/html/rfc5424.
/// [RFC 5427]: https://datatracker.ietf.org/doc/html/rfc5427.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum Facility {
    /// Kernel messages, numerical code 0.
    KERN = 0,
    /// User-level messages, numerical code 1.
    USER = 1,
    /// Mail system, numerical code 2.
    MAIL = 2,
    /// System daemons, numerical code 3.
    DAEMON = 3,
    /// Security/authorization messages, numerical code 4.
    AUTH = 4,
    /// Messages generated internally by syslogd, numerical code 5.
    SYSLOG = 5,
    /// Line printer subsystem, numerical code 6.
    LPR = 6,
    /// Network news subsystem, numerical code 7.
    NEWS = 7,
    /// UUCP subsystem, numerical code 8
    UUCP = 8,
    /// Clock daemon, numerical code 9.
    CRON = 9,
    /// Security/authorization  messages, numerical code 10.
    AUTHPRIV = 10,
    /// FTP daemon, numerical code 11.
    FTP = 11,
    /// NTP subsystem, numerical code 12.
    NTP = 12,
    /// Log audit, numerical code 13.
    AUDIT = 13,
    /// Log alert, numerical code 14.
    ALERT = 14,
    /// Clock daemon, numerical code 15.
    CLOCK = 15,
    /// Reserved for local use, numerical code 16.
    LOCAL0 = 16,
    /// Reserved for local use, numerical code 17.
    LOCAL1 = 17,
    /// Reserved for local use, numerical code 18.
    LOCAL2 = 18,
    /// Reserved for local use, numerical code 19.
    LOCAL3 = 19,
    /// Reserved for local use, numerical code 20.
    LOCAL4 = 20,
    /// Reserved for local use, numerical code 21.
    LOCAL5 = 21,
    /// Reserved for local use, numerical code 22.
    LOCAL6 = 22,
    /// Reserved for local use, numerical code 23.
    LOCAL7 = 23,
}

impl Default for Facility {
    /// Returns the default facility `Facility::USER` as defined in [syslog(3)].
    ///
    /// [syslog(3)]: https://www.man7.org/linux/man-pages/man3/syslog.3.html
    fn default() -> Self {
        Facility::USER
    }
}

impl Facility {
    /// Returns the numerical code of the facility.
    pub fn code(self) -> u8 {
        self as u8
    }

    /// Returns the label of the facility.
    pub fn label(self) -> &'static str {
        match self {
            Facility::KERN => "KERN",
            Facility::USER => "USER",
            Facility::MAIL => "MAIL",
            Facility::DAEMON => "DAEMON",
            Facility::AUTH => "AUTH",
            Facility::SYSLOG => "SYSLOG",
            Facility::LPR => "LPR",
            Facility::NEWS => "NEWS",
            Facility::UUCP => "UUCP",
            Facility::CRON => "CRON",
            Facility::AUTHPRIV => "AUTHPRIV",
            Facility::FTP => "FTP",
            Facility::NTP => "NTP",
            Facility::AUDIT => "AUDIT",
            Facility::ALERT => "ALERT",
            Facility::CLOCK => "CLOCK",
            Facility::LOCAL0 => "LOCAL0",
            Facility::LOCAL1 => "LOCAL1",
            Facility::LOCAL2 => "LOCAL2",
            Facility::LOCAL3 => "LOCAL3",
            Facility::LOCAL4 => "LOCAL4",
            Facility::LOCAL5 => "LOCAL5",
            Facility::LOCAL6 => "LOCAL6",
            Facility::LOCAL7 => "LOCAL7",
        }
    }
}

impl TryFrom<u8> for Facility {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Facility::KERN),
            1 => Ok(Facility::USER),
            2 => Ok(Facility::MAIL),
            3 => Ok(Facility::DAEMON),
            4 => Ok(Facility::AUTH),
            5 => Ok(Facility::SYSLOG),
            6 => Ok(Facility::LPR),
            7 => Ok(Facility::NEWS),
            8 => Ok(Facility::UUCP),
            9 => Ok(Facility::CRON),
            10 => Ok(Facility::AUTHPRIV),
            11 => Ok(Facility::FTP),
            12 => Ok(Facility::NTP),
            13 => Ok(Facility::AUDIT),
            14 => Ok(Facility::ALERT),
            15 => Ok(Facility::CLOCK),
            16 => Ok(Facility::LOCAL0),
            17 => Ok(Facility::LOCAL1),
            18 => Ok(Facility::LOCAL2),
            19 => Ok(Facility::LOCAL3),
            20 => Ok(Facility::LOCAL4),
            21 => Ok(Facility::LOCAL5),
            22 => Ok(Facility::LOCAL6),
            23 => Ok(Facility::LOCAL7),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Facility {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &value.to_lowercase()[..] {
            "kern" => Ok(Facility::KERN),
            "user" => Ok(Facility::USER),
            "mail" => Ok(Facility::MAIL),
            "daemon" => Ok(Facility::DAEMON),
            "auth" => Ok(Facility::AUTH),
            "syslog" => Ok(Facility::SYSLOG),
            "lpr" => Ok(Facility::LPR),
            "news" => Ok(Facility::NEWS),
            "uucp" => Ok(Facility::UUCP),
            "cron" => Ok(Facility::CRON),
            "authpriv" => Ok(Facility::AUTHPRIV),
            "ftp" => Ok(Facility::FTP),
            "ntp" => Ok(Facility::NTP),
            "audit" => Ok(Facility::AUDIT),
            "alert" => Ok(Facility::ALERT),
            "clock" => Ok(Facility::CLOCK),
            "local0" => Ok(Facility::LOCAL0),
            "local1" => Ok(Facility::LOCAL1),
            "local2" => Ok(Facility::LOCAL2),
            "local3" => Ok(Facility::LOCAL3),
            "local4" => Ok(Facility::LOCAL4),
            "local5" => Ok(Facility::LOCAL5),
            "local6" => Ok(Facility::LOCAL6),
            "local7" => Ok(Facility::LOCAL7),
            _ => Err(()),
        }
    }
}

impl FromStr for Facility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl fmt::Display for Facility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}
