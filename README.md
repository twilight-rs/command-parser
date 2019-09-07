[![license badge][]][license link] [![rust badge]][rust link]

# lmao-command-parser

`lmao-command-parser` is a command parser for the [`lmao`] ecosystem.

Included is a mutable configuration that allows you to specify the command
names, prefixes, ignored guilds and users, and more. The parser parses out
commands matching an available command and prefix and provides the command
arguments to you.

### Features

There is a single feature, `model`, which determines whether to support
`lmao_model::channel::Message` as an input. All that this does is additional
checking of the author and the guild it was sent in to see if they're
ignored. If you don't need that, you can disable this feature. With this
feature the dependency tree is 17 dependencies, and without it is 0.

Using `model`:

```toml
[dependencies]
lmao-command-parser = "0.1"
```

Not using `lmao_model`:

```toml
[dependencies]
lmao-command-parser = { default-features = false, version = "0.1" }
```

# Installation

`lmao-command-parser` requires at least Rust 1.36.0.

Add the following to your Cargo.toml:

```toml
[dependencies]
lmao-command-parser = "0.1"
```

### Examples

A simple parser for a bot with one prefix (`"!"`) and two commands: `"echo"`
and `"ping"`:

```rust,no_run
use lmao_command_parser::{Config, Output, Parser};

let mut config = Config::new();

// (Use `Config::add_command` to add a single command)
config.add_command("echo");
config.add_command("ping");

// Add the prefix `"!"`.
// (Use `Config::add_prefixes` to add multiple prefixes)
config.add_prefix("!");

let parser = Parser::new(config);

// Now pass a command to the parser
match parser.parse_str("!echo a message") {
    Output::Command { name: "echo", arguments, .. } => {
        let content = arguments.as_str();

        println!("Got an echo request to send `{}`", content);
    },
    Output::Command { name: "ping", .. } => {
        println!("Got a ping request");
    },
    Output::IgnoredGuild => println!("Message from ignored guild"),
    Output::IgnoredUser => println!("Message from ignored user"),
    Output::NoMatch => println!("Message didn't match a prefix and command"),
    // Ignore all other commands.
    _ => {},
}
```

[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license link]: https://opensource.org/licenses/ISC
[rust badge]: https://img.shields.io/badge/rust-1.36+-93450a.svg?style=flat-square
[rust link]: https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html
[`lmao`]: https://github.com/zeyla/lmao
