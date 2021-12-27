//! # twilight-command-parser
//!
//! [![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]
//!
//! **This crate has been deprecated. Please use [Discord interactions] via the
//! [`twilight-gateway`] or [`twilight-http`] crates.**
//!
//! `twilight-command-parser` is a command parser for the [`twilight-rs`]
//! ecosystem.
//!
//! Included is a mutable configuration that allows you to specify the command
//! names and prefixes. The parser parses out commands matching an available
//! command and prefix and provides the command arguments to you.
//!
//! ### Examples
//!
//! A simple parser for a bot with one prefix (`"!"`) and two commands: `"echo"`
//! and `"ping"`:
//!
//! ```rust,no_run
//! use twilight_command_parser::{Command, CommandParserConfig, Parser};
//!
//! let mut config = CommandParserConfig::new();
//!
//! config.add_command("echo", false);
//! config.add_command("ping", false);
//!
//! // Add the prefix `"!"`.
//! // (Use `CommandParserConfig::add_prefixes` to add multiple prefixes)
//! config.add_prefix("!");
//!
//! let parser = Parser::new(config);
//!
//! // Now pass a command to the parser
//! match parser.parse("!echo a message") {
//!     Some(Command { name: "echo", arguments, .. }) => {
//!         let content = arguments.as_str();
//!
//!         println!("Got an echo request to send `{}`", content);
//!     },
//!     Some(Command { name: "ping", .. }) => {
//!         println!("Got a ping request");
//!     },
//!     // Ignore all other commands.
//!     Some(_) => {},
//!     None => println!("Message didn't match a prefix and command"),
//! }
//! ```
//!
//! [Discord interactions]: https://discord.com/developers/docs/interactions/application-commands
//! [codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
//! [codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
//! [discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
//! [discord link]: https://discord.gg/7jj8n7D
//! [github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
//! [github link]: https://github.com/twilight-rs/twilight
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
//! [license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
//! [rust badge]: https://img.shields.io/badge/rust-1.53+-93450a.svg?style=for-the-badge&logo=rust
//! [`twilight-gateway`]: https://crates.io/crates/twilight-gateway
//! [`twilight-http`]: https://crates.io/crates/twilight-http
//! [`twilight-rs`]: https://github.com/twilight-rs/twilight

#![deprecated(
    since = "0.8.1",
    note = "use interactions via `twilight-http` or `twilight-gateway`"
)]
#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code,
    unused,
    warnings
)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

pub mod config;

mod arguments;
mod casing;
mod parser;

pub use self::{
    arguments::Arguments,
    config::CommandParserConfig,
    parser::{Command, Parser},
};
