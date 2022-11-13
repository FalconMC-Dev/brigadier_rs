# brigadier-rs

[![Latest Version](https://img.shields.io/crates/v/brigadier_rs?style=for-the-badge)](https://crates.io/crates/brigadier_rs)
[![Docs.rs](https://img.shields.io/docsrs/brigadier_rs?style=for-the-badge)](https://docs.rs/brigadier_rs/latest/brigadier_rs/)

This crate is a parsing library for Minecraft commands inspired
by [Mojang's brigadier](https://github.com/Mojang/Brigadier). It was developed
for use with [`FalconMC`](https://github.com/FalconMC-Dev/FalconMC) but can
be used fully indepedently (hence the name).

## Features

Unlike Mojang's Brigadier, this library does *not* make use of nodes internally.
Instead, all command parsers are strong-typed and consist of chained types. This
allows for fewer allocations than the java version (which uses dynamic arrays (`Vec`)
internally.

Even though it is currently not implemented, it is desired to provide a way to
build a node tree from such chained parsers so command definitions can be exchanged
between servers and clients using the protocol.

### Creating a parser

Using the builder pattern, different parsers can be chained together to create
a logical tree a command can be propagated through. This library is heavily
inspired by [`nom`](https://crates.io/crates/nom) and uses it for all its agument
parsers. See nom for more information.

For example:

```rust
let parser = literal("foo")
    .then(
        integer_i32("bar")
            .build_exec(|ctx, bar| {
                println!("Bar is {}", bar);
                Ok::<(), Infallible>(())
            })
    ).build_exec(|ctx| {
        println!("Called foo with no arguments");
        Ok::<(), Infallible>(())
    });
```

This snippet creates a new parser that can parse commands in the forms of `foo`
and `foo <bar>` and can be represented in a tree like this:

```ditaa
              +-----------+       +---------+
              | i32 (bar) +-----> | Execute |
              +-----+-----+       +---------+
                    ^
                    |
                +---+------+
+-----------+   |    (foo) |      +---------+
| lit (foo) +-->| Then     +----> | Execute |
+-----------+   +----------+      +---------+
```

The parser first expects a literal string "foo" as denoted by the `literal("foo")`.
After this literal value, an optional integer can be provided. Important to note
is that this second argument is optional because there is a `build_execute`
call on both the parent of the second argument as well as the second argument
itself.

Unlike Mojang's brigadier, arguments are not collected in a `Context` object.
They are instead fed directly into the provided closures. A generic context
however is provided so dependents can pass data to the closures after parsing.

### Command help

A `HelpArgument` is provided to easily integrate a command into a help system.
This is done by calling `help()` on a command parser like this:

```rust
let parser = literal("foo")
    .then(
        integer_i32("bar")
            .build_exec(|ctx, bar| {
                println!("Bar is {}", bar);
                Ok::<(), Infallible>(())
            })
    ).build_exec(|ctx| {
        println!("Called foo with no arguments");
        Ok::<(), Infallible>(())
    })
    .help("Short description of foo")
    .build_exec(|ctx, usages| {
        println!("'foo help' was called");
        Ok::<(), Infallible>(())
    });
```

The parser can now return `foo` and `Short description of foo` when asked,
this is useful for collecting a list of commands. This also automatically chains a parser
for `foo help`. The `usages` variable is an iterator over all the different syntaxes
this parser understands. In this example, that would be:

- `foo`
- `foo <bar>`

There will be as many syntaxes as action points (`build_exec` or`build_propagate`)
defined. Note that `foo help` is ignored.

### Node structure

More information on node structures will follow at a later date.
Open for contributions.

## License

Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Please feel free to contribute!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
