#![allow(
    clippy::cognitive_complexity,
    clippy::large_enum_variant,
    clippy::module_inception,
    clippy::needless_doctest_main
)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(unused_must_use)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

//! A library for working with connected UDP sockets.
//!
//! The `connected_udp` crate provides safer and more consistent APIs for
//! connected UDP sockets. For example, there's inconsistent behavior between
//! operating systems when `sendto()` has been called on a socket where
//! `connect()` was previously called.
//!
//! # `ConnectedUdpSocket`
//!
//! [`ConnectedUdpSocket`] allows you to work with connected UDP sockets from
//! standard library types.
//!
//! See the [struct docs](`ConnectedUdpSocket`) for more details.

mod connected_udp;

pub use connected_udp::ConnectedUdpSocket;
