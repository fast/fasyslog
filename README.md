# Fasyslog: A fast syslog client written in Rust

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MSRV 1.75][msrv-badge]](https://www.whatrustisit.com)
[![Apache 2.0 licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/fasyslog.svg
[crates-url]: https://crates.io/crates/fasyslog
[docs-badge]: https://docs.rs/fasyslog/badge.svg
[msrv-badge]: https://img.shields.io/badge/MSRV-1.75-green?logo=rust
[docs-url]: https://docs.rs/fasyslog
[license-badge]: https://img.shields.io/crates/l/fasyslog
[license-url]: LICENSE
[actions-badge]: https://github.com/fast/fasyslog/workflows/CI/badge.svg
[actions-url]:https://github.com/fast/fasyslog/actions?query=workflow%3ACI

## Description

Client library written in Rust to send messages to a Syslog server. Support implementations:

* RFC-3164 Formatter: [The BSD syslog Protocol](https://datatracker.ietf.org/doc/html/rfc3164)
* RFC-5424 Formatter: [The Syslog Protocol](https://datatracker.ietf.org/doc/html/rfc5424)
* `UdpSender`: [RFC 5426 - Transmission of Syslog Messages over UDP](https://datatracker.ietf.org/doc/html/rfc5426)
* `TcpSender`: [RFC 6587 - Transmission of Syslog Messages over TCP](https://datatracker.ietf.org/doc/html/rfc6587)
* `NativeTlsSender`: [RFC 5425 - Transport Layer Security (TLS) Transport Mapping for Syslog](https://datatracker.ietf.org/doc/html/rfc5425)
  * This implementation is based on [`native-tls`](https://crates.io/crates/native-tls) and requires features `native-tls` turned on.
* `RustlsSender`: [RFC 5425 - Transport Layer Security (TLS) Transport Mapping for Syslog](https://datatracker.ietf.org/doc/html/rfc5425)
    * This implementation is based on [`rustls`](https://crates.io/crates/rustls) and requires features `rustls` turned on.
* (unix only) Unix domain socket sender (datagram or stream)

## Getting Started

Add `fasyslog` to your `Cargo.toml`:

```shell
cargo add fasyslog
```

```rust
use fasyslog::Severity;

fn main() {
    let mut sender = fasyslog::sender::tcp_well_known().unwrap();
    let message = format!("Hello, fasyslog!");
    // send a message with RFC 3164 format
    sender.send_rfc3164(Severity::ERROR, message).unwrap();
    sender.flush().unwrap();

    // send a message with RFC 5424 format
    const EMPTY_MSGID: Option<&str> = None;
    const EMPTY_STRUCTURED_DATA: Vec<fasyslog::SDElement> = Vec::new();
    sender.send_rfc5424(Severity::ERROR, EMPTY_MSGID, EMPTY_STRUCTURED_DATA, message).unwrap();
    sender.flush().unwrap();
}
```

If you'd like to integrate with `log` crate, you can try the `logforth` example:

```toml
[dependencies]
log = { version = "..." }
logforth = { version = "...", features = ["syslog"] }
```

```rust
use logforth::append::syslog;
use logforth::append::syslog::Syslog;
use logforth::append::syslog::SyslogWriter;

fn main() {
    let syslog_writer = SyslogWriter::tcp_well_known().unwrap();
    let (non_blocking, _guard) = syslog::non_blocking(syslog_writer).finish();

    logforth::builder()
        .dispatch(|d| {
            d.filter(log::LevelFilter::Trace)
                .append(Syslog::new(non_blocking))
        })
        .apply();

    log::info!("This log will be written to syslog.");
}
```

## Example

Check the [examples](examples) directory for more examples.

## Documentation

Read the online documents at https://docs.rs/fasyslog.

## Minimum Supported Rust Version (MSRV)

This crate is built against the latest stable release, and its minimum supported rustc version is 1.75.0.

The policy is that the minimum Rust version required to use this crate can be increased in minor version updates. For example, if Fasyslog 1.0 requires Rust 1.20.0, then Fasyslog 1.0.z for all values of z will also require Rust 1.20.0 or newer. However, Fasyslog 1.y for y > 0 may require a newer minimum version of Rust.

## License

This project is licensed under [Apache License, Version 2.0](LICENSE).
