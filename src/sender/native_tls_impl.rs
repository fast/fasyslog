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

pub extern crate native_tls;

use std::borrow::Cow;
use std::io;
use std::io::BufWriter;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

use native_tls::TlsConnector;
use native_tls::TlsConnectorBuilder;
use native_tls::TlsStream;

use crate::format::SyslogContext;
use crate::sender::internal::impl_syslog_sender_common;
use crate::sender::internal::impl_syslog_stream_send_formatted;

/// Create a TLS sender that sends messages to the well-known port (6514).
///
/// See also [RFC-5425] §4.1 Port Assignment.
///
/// [RFC-5425]: https://datatracker.ietf.org/doc/html/rfc5425#section-4.1
pub fn native_tls_well_known<S: AsRef<str>>(domain: S) -> io::Result<NativeTlsSender> {
    let domain = domain.as_ref();
    native_tls(format!("{domain}:6514"), domain)
}

/// Create a TLS sender that sends messages to the given address.
pub fn native_tls<A: ToSocketAddrs, S: AsRef<str>>(
    addr: A,
    domain: S,
) -> io::Result<NativeTlsSender> {
    native_tls_with(addr, domain, TlsConnector::builder())
}

/// Create a TLS sender that sends messages to the given address with certificate builder.
pub fn native_tls_with<A: ToSocketAddrs, S: AsRef<str>>(
    addr: A,
    domain: S,
    builder: TlsConnectorBuilder,
) -> io::Result<NativeTlsSender> {
    NativeTlsSender::connect(addr, domain, builder)
}

/// A syslog sender that sends messages to a TCP socket over TLS.
///
/// Users can obtain a `NativeTlsSender` by calling [`native_tls_well_known()`], [`native_tls()`],
/// or [`native_tls_with()`].
#[derive(Debug)]
pub struct NativeTlsSender {
    writer: BufWriter<TlsStream<TcpStream>>,
    context: SyslogContext,
    postfix: Cow<'static, str>,
}

impl NativeTlsSender {
    /// Connect to a TCP socket over TLS at the given address.
    pub fn connect<A: ToSocketAddrs, S: AsRef<str>>(
        addr: A,
        domain: S,
        builder: TlsConnectorBuilder,
    ) -> io::Result<Self> {
        let domain = domain.as_ref();
        let stream = TcpStream::connect(addr)?;
        let connector = builder.build().map_err(io::Error::other)?;
        let stream = connector
            .connect(domain, stream)
            .map_err(io::Error::other)?;
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
}

impl_syslog_sender_common!(NativeTlsSender);
impl_syslog_stream_send_formatted!(NativeTlsSender);
