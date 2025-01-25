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

//! Send syslog messages to a syslog server.

use std::fmt;
use std::io;

use crate::SDElement;
use crate::Severity;

#[cfg(unix)]
mod unix_impl;
#[cfg(unix)]
pub use unix_impl::*;

#[cfg(feature = "native-tls")]
mod native_tls_impl;
#[cfg(feature = "native-tls")]
pub use native_tls_impl::*;

#[cfg(feature = "rustls")]
mod rustls_impl;
#[cfg(feature = "rustls")]
pub use rustls_impl::*;

mod tcp_impl;
pub use tcp_impl::*;

mod udp_impl;
pub use udp_impl::*;

pub(crate) mod internal;

/// Static dispatch for the different sender types.
#[derive(Debug)]
pub enum SyslogSender {
    Tcp(TcpSender),
    Udp(UdpSender),
    #[cfg(feature = "native-tls")]
    NativeTlsSender(NativeTlsSender),
    #[cfg(feature = "rustls")]
    RustlsSender(Box<RustlsSender>),
    #[cfg(unix)]
    UnixDatagram(UnixDatagramSender),
    #[cfg(unix)]
    UnixStream(UnixStreamSender),
}

impl SyslogSender {
    /// Send a message with the given severity as defined in RFC-3164.
    pub fn send_rfc3164<M: fmt::Display>(
        &mut self,
        severity: Severity,
        message: M,
    ) -> io::Result<()> {
        match self {
            SyslogSender::Tcp(sender) => sender.send_rfc3164(severity, message),
            SyslogSender::Udp(sender) => sender.send_rfc3164(severity, message),
            #[cfg(feature = "native-tls")]
            SyslogSender::NativeTlsSender(sender) => sender.send_rfc3164(severity, message),
            #[cfg(feature = "rustls")]
            SyslogSender::RustlsSender(sender) => sender.send_rfc3164(severity, message),
            #[cfg(unix)]
            SyslogSender::UnixDatagram(sender) => sender.send_rfc3164(severity, message),
            #[cfg(unix)]
            SyslogSender::UnixStream(sender) => sender.send_rfc3164(severity, message),
        }
    }

    /// Send a message with the given severity as defined in RFC-5424.
    pub fn send_rfc5424<S: Into<String>, M: fmt::Display>(
        &mut self,
        severity: Severity,
        msgid: Option<S>,
        elements: Vec<SDElement>,
        message: M,
    ) -> io::Result<()> {
        match self {
            SyslogSender::Tcp(sender) => sender.send_rfc5424(severity, msgid, elements, message),
            SyslogSender::Udp(sender) => sender.send_rfc5424(severity, msgid, elements, message),
            #[cfg(feature = "native-tls")]
            SyslogSender::NativeTlsSender(sender) => {
                sender.send_rfc5424(severity, msgid, elements, message)
            }
            #[cfg(feature = "rustls")]
            SyslogSender::RustlsSender(sender) => {
                sender.send_rfc5424(severity, msgid, elements, message)
            }
            #[cfg(unix)]
            SyslogSender::UnixDatagram(sender) => {
                sender.send_rfc5424(severity, msgid, elements, message)
            }
            #[cfg(unix)]
            SyslogSender::UnixStream(sender) => {
                sender.send_rfc5424(severity, msgid, elements, message)
            }
        }
    }

    /// Send a pre-formatted message.
    pub fn send_formatted(&mut self, formatted: &[u8]) -> io::Result<()> {
        match self {
            SyslogSender::Tcp(sender) => sender.send_formatted(formatted),
            SyslogSender::Udp(sender) => sender.send_formatted(formatted),
            #[cfg(feature = "native-tls")]
            SyslogSender::NativeTlsSender(sender) => sender.send_formatted(formatted),
            #[cfg(feature = "rustls")]
            SyslogSender::RustlsSender(sender) => sender.send_formatted(formatted),
            #[cfg(unix)]
            SyslogSender::UnixDatagram(sender) => sender.send_formatted(formatted),
            #[cfg(unix)]
            SyslogSender::UnixStream(sender) => sender.send_formatted(formatted),
        }
    }

    /// Flush the underlying writer if needed.
    ///
    /// When the underlying writer is a streaming writer (TCP, UnixStream, etc.), periodically
    /// flush is essential to ensure that the message is sent to the syslog server [1].
    ///
    /// When the underlying writer is a datagram writer (UDP, UnixDatagram, etc.), flush is a no-op,
    /// and every call to `send_xxx` defines the boundary of the packet.
    ///
    /// [1]: https://github.com/Geal/rust-syslog/issues/69
    pub fn flush(&mut self) -> io::Result<()> {
        match self {
            SyslogSender::Tcp(sender) => sender.flush(),
            SyslogSender::Udp(_) => Ok(()),
            #[cfg(feature = "native-tls")]
            SyslogSender::NativeTlsSender(sender) => sender.flush(),
            #[cfg(feature = "rustls")]
            SyslogSender::RustlsSender(sender) => sender.flush(),
            #[cfg(unix)]
            SyslogSender::UnixDatagram(_) => Ok(()),
            #[cfg(unix)]
            SyslogSender::UnixStream(sender) => sender.flush(),
        }
    }
}
