## Rust Examples

#### About Cargo

Cargo let's you build all your Rust files into the format you require.

Cargo.toml defines your app config.

The `1.0/cargo_example` has 2 tables: `package` and `bin`.

- The first contains metadata about the project
- The second tells Cargo we're interested in building a binary, not a library (though we could do both!), as well as what it is named.

#### Using Cargo

```
$ cargo build
$ ./target/hello_world
```
