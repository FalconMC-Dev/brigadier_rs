# brigadier-rs

This crate is a parsing library for Minecraft commands inspired
by [Mojang's brigadier](https://github.com/Mojang/Brigadier). It was developed
for use with [`FalconMC`](https://github.com/FalconMC-Dev/FalconMC) but can
be used fully indepedently (hence the name).

## Usage

Unlike Mojang's Brigadier, this library does *not* make use of nodes internally.
Instead, all command parsers are strong-typed and consist of chained types. This
allows for less allocations than the java version (which uses dynamic arrays (`Vec`)
internally.

Even though it is currently not implemented, it is planned to provide a way to
build a node tree from such chained parsers so it can be exchanged between
servers and clients for example.

### Creating a parser

Using the builder pattern, different parsers can be chained together to create
a logical tree a command can be propagated through.

For example:

```rust
let parser = literal("foo")
    .then(
        integer_i32("bar")
            .build_execute(|bar| {
                println!("Bar is {}", bar);
                Ok::<bool, Infallible>(true)
            })
    ).build_execute(|| {
        println!("Called foo with no arguments");
        Ok::<bool, Infallible>(true)
    });
```

This snippet creates a new parser that can parse commands in the forms of `foo`
and `foo <bar>` and can be represented in a tree as follows:

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
They are instead fed directly into the provided closures. In a future version,
a context trait will be provided so library users can define their own contexts
(useful for implementing callbacks for example).

### Node structure

More information on node structures will follow at a later date.

### Command help

More information on command usage will follow at a later date.

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
