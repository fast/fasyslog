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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! `fasyslog` is a fast syslog client written in Rust.
//!
//! # Overview
//!
//! This crate provides facilities to send log messages via syslog. Support implementations:
//!
//! * [RFC-3164 Formatter]: [The BSD syslog Protocol](https://datatracker.ietf.org/doc/html/rfc3164)
//! * [RFC-5424 Formatter]: [The Syslog Protocol](https://datatracker.ietf.org/doc/html/rfc5424)
//! * [`UdpSender`]: [RFC 5426 - Transmission of Syslog Messages over UDP](https://datatracker.ietf.org/doc/html/rfc5426)
//! * [`TcpSender`]: [RFC 6587 - Transmission of Syslog Messages over TCP](https://datatracker.ietf.org/doc/html/rfc6587)
//! * (unix only) Unix domain socket sender (datagram or stream)
//!
//! [RFC-3164 Formatter]: format::RFC3164Formatter
//! [RFC-5424 Formatter]: format::RFC5424Formatter
//! [`UdpSender`]: sender::UdpSender
//! [`TcpSender`]: sender::TcpSender
//!
//! # Example
//!
//! Send a message to a remote syslog server:
//!
//! ```rust, no_run
//! let mut sender = fasyslog::sender::tcp_well_known().unwrap();
//! sender
//!     .send_rfc3164(fasyslog::Severity::INFORMATIONAL, "Hello, syslog!")
//!     .unwrap();
//! sender.flush().unwrap();
//!
//! let mut element = fasyslog::SDElement::new("exampleSDID@32473").unwrap();
//! element.add_param("iut", "3").unwrap();
//! sender
//!     .send_rfc5424(
//!         fasyslog::Severity::NOTICE,
//!         Some("TCPIN"),
//!         vec![element],
//!         "Hello, syslog!",
//!     )
//!     .unwrap();
//! sender.flush().unwrap();
//! ```

mod facility;
pub use facility::*;

mod severity;
pub use severity::*;

mod structured_data;
pub use structured_data::*;

pub mod format;
pub mod sender;

mod internal;
