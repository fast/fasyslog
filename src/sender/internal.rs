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

macro_rules! impl_stream_syslog_sender {
    ($sender:ident, $stream:ident) => {
        impl $sender {
            /// Send a message with the given severity as defined in RFC-3164.
            pub fn send_rfc3164<M: std::fmt::Display>(
                &mut self,
                severity: $crate::Severity,
                message: M,
            ) -> std::io::Result<()> {
                use std::io::Write;
                let message = self.context.format_rfc3164(severity, Some(message));
                write!(&mut self.$stream, "{message}")
            }

            /// Send a message with the given severity as defined in RFC-5424.
            pub fn send_rfc5424<S: Into<String>, M: std::fmt::Display>(
                &mut self,
                severity: $crate::Severity,
                msgid: Option<S>,
                elements: Vec<$crate::SDElement>,
                message: M,
            ) -> std::io::Result<()> {
                use std::io::Write;
                let message = self
                    .context
                    .format_rfc5424(severity, msgid, elements, Some(message));
                write!(&mut self.$stream, "{message}")
            }

            /// Flush the writer.
            pub fn flush(&mut self) -> io::Result<()> {
                use std::io::Write;
                self.writer.flush()
            }
        }
    };
}

pub(crate) use impl_stream_syslog_sender;

macro_rules! impl_datagram_syslog_sender {
    ($sender:ident, $socket:ident) => {
        impl $sender {
            /// Send a message with the given severity as defined in RFC-3164.
            pub fn send_rfc3164<M: std::fmt::Display>(
                &mut self,
                severity: $crate::Severity,
                message: M,
            ) -> std::io::Result<()> {
                let message = self.context.format_rfc3164(severity, Some(message));
                self.$socket.send(message.to_string().as_bytes())?;
                Ok(())
            }

            /// Send a message with the given severity as defined in RFC-5424.
            pub fn send_rfc5424<S: Into<String>, M: std::fmt::Display>(
                &mut self,
                severity: $crate::Severity,
                msgid: Option<S>,
                elements: Vec<$crate::SDElement>,
                message: M,
            ) -> std::io::Result<()> {
                let message = self
                    .context
                    .format_rfc5424(severity, msgid, elements, Some(message));
                self.$socket.send(message.to_string().as_bytes())?;
                Ok(())
            }
        }
    };
}

pub(crate) use impl_datagram_syslog_sender;
