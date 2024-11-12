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
use std::os::unix::net::UnixDatagram;
use std::os::unix::net::UnixStream;
use std::path::Path;

use crate::format::SyslogContext;
use crate::sender::internal::impl_syslog_sender_common;
use crate::sender::SyslogSender;

/// Create a Unix datagram sender that sends messages to the given path.
pub fn unix_datagram(path: impl AsRef<Path>) -> io::Result<UnixDatagramSender> {
    UnixDatagramSender::connect(path)
}

/// Create a Unix stream sender that sends messages to the given path.
pub fn unix_stream(path: impl AsRef<Path>) -> io::Result<UnixStreamSender> {
    UnixStreamSender::connect(path)
}

/// Create a Unix sender that sends messages to the given path.
///
/// Automatically chooses between Unix datagram and Unix stream based on the path.
pub fn unix(path: impl AsRef<Path>) -> io::Result<SyslogSender> {
    const EPROTOTYPE: i32 = nix::errno::Errno::EPROTOTYPE as i32;
    let path = path.as_ref();
    match unix_datagram(path) {
        Ok(sender) => Ok(SyslogSender::UnixDatagram(sender)),
        Err(err) => match err.raw_os_error() {
            Some(EPROTOTYPE) => unix_stream(path).map(SyslogSender::UnixStream),
            _ => Err(err),
        },
    }
}

pub fn unix_well_known() -> io::Result<SyslogSender> {
    cfg_if::cfg_if! {
       if #[cfg(target_os = "macos")] {
            // NOTE: This may not work on Monterey (12.x) and above,
            //  see also https://github.com/python/cpython/issues/91070.
            unix("/var/run/syslog")
        } else if #[cfg(target_os = "freebsd")] {
            unix("/var/run/log")
        } else {
            unix("/dev/log")
        }
    }
}

/// A syslog sender that sends messages to a Unix datagram socket.
///
/// Typically, this sender is used to send messages to the local syslog daemon. Typical paths are
/// `/dev/log`, `/var/run/syslog`, or `/var/run/log`.
#[derive(Debug)]
pub struct UnixDatagramSender {
    socket: UnixDatagram,
    context: SyslogContext,
}

impl UnixDatagramSender {
    /// Connect to a Unix datagram socket at the given path.
    pub fn connect(path: impl AsRef<Path>) -> io::Result<Self> {
        let socket = UnixDatagram::unbound()?;
        socket.connect(path)?;
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

    /// Send a pre-formatted message.
    pub fn send_formatted(&mut self, formatted: &[u8]) -> io::Result<()> {
        self.socket.send(formatted)?;
        Ok(())
    }
}

impl_syslog_sender_common!(UnixDatagramSender);

/// A syslog sender that sends messages to a Unix stream socket.
///
/// Typically, this sender is used to send messages to the local syslog daemon. Typical paths are
/// `/dev/log`, `/var/run/syslog`, or `/var/run/log`.
#[derive(Debug)]
pub struct UnixStreamSender {
    writer: BufWriter<UnixStream>,
    context: SyslogContext,
    postfix: Cow<'static, str>,
}

impl UnixStreamSender {
    /// Connect to a Unix stream socket at the given path.
    pub fn connect(path: impl AsRef<Path>) -> io::Result<Self> {
        let socket = UnixStream::connect(path)?;
        Ok(Self {
            writer: BufWriter::new(socket),
            context: SyslogContext::default(),
            postfix: Cow::Borrowed("\r\n"),
        })
    }

    /// Set the postfix when formatting Syslog message.
    ///
    /// Default is "\r\n". You can use empty string to set no postfix.
    pub fn set_postfix(&mut self, postfix: impl Into<Cow<'static, str>>) {
        self.postfix = postfix.into();
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

impl_syslog_sender_common!(UnixStreamSender);
