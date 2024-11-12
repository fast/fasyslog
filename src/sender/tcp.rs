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

use std::borrow::Cow;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

use crate::format::SyslogContext;
use crate::sender::internal::impl_syslog_sender_common;

/// Create a TCP sender that sends messages to the well-known port (601).
///
/// See also [RFC-3195] §9.2 The System (Well-Known) TCP port number for syslog-conn.
///
/// [RFC-3195]: https://datatracker.ietf.org/doc/html/rfc3195#section-9.2
pub fn tcp_well_known() -> io::Result<TcpSender> {
    tcp("127.0.0.1:601")
}

/// Create a TCP sender that sends messages to the given address.
pub fn tcp<A: ToSocketAddrs>(addr: A) -> io::Result<TcpSender> {
    TcpSender::connect(addr)
}

/// A syslog sender that sends messages to a TCP socket.
#[derive(Debug)]
pub struct TcpSender {
    writer: BufWriter<TcpStream>,
    context: SyslogContext,
    postfix: Cow<'static, str>,
}

impl TcpSender {
    /// Connect to a TCP socket at the given address.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            writer: BufWriter::new(stream),
            context: SyslogContext::default(),
            postfix: Cow::Borrowed("\r\n"),
        })
    }

    /// Set the postfix when formatting Syslog message.
    ///
    /// This is generally '\r\n' as defined in [RFC-6587] §3.4.2.
    ///
    /// [RFC-6587]: https://datatracker.ietf.org/doc/html/rfc6587
    pub fn set_postfix(&mut self, postfix: impl Into<Cow<'static, str>>) {
        self.postfix = postfix.into();
    }

    /// Set the context when formatting Syslog message.
    pub fn set_context(mut self, context: SyslogContext) {
        self.context = context;
    }

    /// Mutate the context when formatting Syslog message.
    pub fn mut_context(&mut self) -> &mut SyslogContext {
        &mut self.context
    }

    /// Send a pre-formatted message.
    pub fn send_formatted(&mut self, formatted: &[u8]) -> io::Result<()> {
        self.writer.write_all(formatted)?;
        self.writer.write_all(self.postfix.as_bytes())?;
        Ok(())
    }

    /// Flush the writer.
    pub fn flush(&mut self) -> io::Result<()> {
        use std::io::Write;
        self.writer.flush()
    }
}

impl_syslog_sender_common!(TcpSender);
