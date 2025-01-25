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

pub extern crate rustls;

use std::borrow::Cow;
use std::io;
use std::io::BufWriter;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use rustls::pki_types::ServerName;
use rustls::ClientConfig;
use rustls::ClientConnection;
use rustls::RootCertStore;
use rustls::StreamOwned;

use crate::format::SyslogContext;
use crate::sender::internal::impl_syslog_sender_common;
use crate::sender::internal::impl_syslog_stream_send_formatted;

/// Create a TLS sender that sends messages to the well-known port (6514).
///
/// See also [RFC-5425] §4.1 Port Assignment.
///
/// [RFC-5425]: https://datatracker.ietf.org/doc/html/rfc5425#section-4.1
pub fn rustls_well_known<S: Into<String>>(domain: S) -> io::Result<RustlsSender> {
    let domain = domain.into();
    rustls(format!("{domain}:6514"), domain)
}

/// Create a TLS sender that sends messages to the given address.
pub fn rustls<A: ToSocketAddrs, S: Into<String>>(addr: A, domain: S) -> io::Result<RustlsSender> {
    let mut roots = RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().certs {
        roots.add(cert).unwrap();
    }

    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    rustls_with(addr, domain, Arc::new(config))
}

/// Create a TLS sender that sends messages to the given address with certificate builder.
pub fn rustls_with<A: ToSocketAddrs, S: Into<String>>(
    addr: A,
    domain: S,
    config: Arc<ClientConfig>,
) -> io::Result<RustlsSender> {
    RustlsSender::connect(addr, domain, config)
}

/// A syslog sender that sends messages to a TCP socket over TLS.
///
/// Users can obtain a `RustlsSender` by calling [`rustls_well_known()`], [`rustls()`],
/// or [`rustls_with()`].
#[derive(Debug)]
pub struct RustlsSender {
    writer: BufWriter<StreamOwned<ClientConnection, TcpStream>>,
    context: SyslogContext,
    postfix: Cow<'static, str>,
}

impl RustlsSender {
    /// Connect to a TCP socket over TLS at the given address.
    pub fn connect<A: ToSocketAddrs, S: Into<String>>(
        addr: A,
        domain: S,
        config: Arc<ClientConfig>,
    ) -> io::Result<Self> {
        let domain = domain.into();
        let domain = ServerName::try_from(domain).map_err(io::Error::other)?;
        let stream = TcpStream::connect(addr)?;
        let conn = ClientConnection::new(config, domain).map_err(io::Error::other)?;
        let stream = StreamOwned::new(conn, stream);
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

impl_syslog_sender_common!(RustlsSender);
impl_syslog_stream_send_formatted!(RustlsSender);
