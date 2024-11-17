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
use crate::sender::internal::impl_syslog_sender_common;

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

/// Create a UDP sender that broadcast messages to the well-known port (514).
///
/// See also [RFC-3164] §2 Transport Layer Protocol.
///
/// [RFC-3164]: https://datatracker.ietf.org/doc/html/rfc3164#section-2
pub fn broadcast_well_known() -> io::Result<UdpSender> {
    broadcast(514)
}

/// Create a UDP sender that broadcast messages
pub fn broadcast(port: u16) -> io::Result<UdpSender> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.connect(format!("255.255.255.255:{port}"))?;
    Ok(UdpSender::new(socket))
}

/// A syslog sender that sends messages to a UDP socket.
///
/// Users can obtain a `UdpSender` by calling [`udp_well_known`], [`udp`], [`broadcast_well_known`],
/// or [`broadcast`].
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
        Ok(Self::new(socket))
    }

    /// Create a new UDP sender with the given socket and remote address.
    ///
    /// This is useful when users want to configure the socket in fine-grained. Note that the
    /// passed `socket` MUST be connected to the remote address.
    pub fn new(socket: UdpSocket) -> Self {
        Self {
            socket,
            context: SyslogContext::default(),
        }
    }

    /// Set the context when formatting Syslog message.
    pub fn set_context(&mut self, context: SyslogContext) {
        self.context = context;
    }

    /// Mutate the context when formatting Syslog message.
    pub fn mut_context(&mut self) -> &mut SyslogContext {
        &mut self.context
    }

    /// Send a pre-formatted message.
    pub fn send_formatted(&mut self, formatted: &[u8]) -> io::Result<()> {
        self.socket.send(formatted)?;
        Ok(())
    }
}

impl_syslog_sender_common!(UdpSender);
