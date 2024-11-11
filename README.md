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

* RFC-3164 Formatter: [The BSD syslog Protocol](http://tools.ietf.org/html/rfc3164)
* RFC-5424 Formatter: [The Syslog Protocol](http://tools.ietf.org/html/rfc5424)
* `UdpSender`: [RFC 5426 - Transmission of Syslog Messages over UDP](http://tools.ietf.org/html/rfc5426)
* `TcpSender`: [RFC 6587 - Transmission of Syslog Messages over TCP](http://tools.ietf.org/html/rfc6587)

## Getting Started

Add `fasyslog` to your `Cargo.toml`:

```shell
cargo add fasyslog
```

## Example

Check the [examples](examples) directory for more examples.

## Documentation

Read the online documents at https://docs.rs/logforth.

## Minimum Supported Rust Version (MSRV)

This crate is built against the latest stable release, and its minimum supported rustc version is 1.75.0.

The policy is that the minimum Rust version required to use this crate can be increased in minor version updates. For example, if Fasyslog 1.0 requires Rust 1.20.0, then Fasyslog 1.0.z for all values of z will also require Rust 1.20.0 or newer. However, Fasyslog 1.y for y > 0 may require a newer minimum version of Rust.

## License

This project is licensed under [Apache License, Version 2.0](LICENSE).
