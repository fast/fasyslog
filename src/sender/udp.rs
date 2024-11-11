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

use std::io;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

use crate::format::SyslogContext;
use crate::SDElement;
use crate::Severity;

/// Create a UDP sender that sends messages to the well-known port (514).
///
/// See also [RFC-3164] §2 Transport Layer Protocol.
///
/// [RFC-3164]: https://datatracker.ietf.org/doc/html/rfc3164#section-2
pub fn udp_well_known() -> io::Result<UdpSender> {
    udp("0.0.0.0:0", "127.0.0.1:514")
}

/// Create a UDP sender that sends messages to the given address.
pub fn udp<L: ToSocketAddrs, R: ToSocketAddrs>(local: L, remote: R) -> io::Result<UdpSender> {
    UdpSender::connect(local, remote)
}

/// A syslog sender that sends messages to a UDP socket.
#[derive(Debug)]
pub struct UdpSender {
    socket: UdpSocket,
    context: SyslogContext,
}

impl UdpSender {
    /// Connect to a UDP socket at the given address.
    pub fn connect<L: ToSocketAddrs, R: ToSocketAddrs>(local: L, remote: R) -> io::Result<Self> {
        let socket = UdpSocket::bind(local)?;
        socket.connect(remote)?;
        Ok(Self {
            socket,
            context: SyslogContext::default(),
        })
    }

    /// Set the context when formatting Syslog message.
    pub fn set_context(&mut self, context: SyslogContext) {
        self.context = context;
    }

    /// Mutate the context when formatting Syslog message.
    pub fn mut_context(&mut self) -> &mut SyslogContext {
        &mut self.context
    }

    /// Send a message with the given severity as defined in RFC-3164.
    pub fn send_rfc3164<M: std::fmt::Display>(
        &mut self,
        severity: Severity,
        message: M,
    ) -> io::Result<()> {
        let message = self.context.format_rfc3164(severity, Some(message));
        self.socket.send(message.to_string().as_bytes())?;
        Ok(())
    }

    /// Send a message with the given severity as defined in RFC-5424.
    pub fn send_rfc5424<S: Into<String>, M: std::fmt::Display>(
        &mut self,
        severity: Severity,
        msgid: Option<S>,
        elements: Vec<SDElement>,
        message: M,
    ) -> io::Result<()> {
        let message = self
            .context
            .format_rfc5424(severity, msgid, elements, Some(message));
        self.socket.send(message.to_string().as_bytes())?;
        Ok(())
    }
}
