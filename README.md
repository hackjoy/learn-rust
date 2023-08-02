## Learn Rust

#### Cargo

Cargo is tool for Rust providing:

- **Dependency management**: declare and manage dependencies.
- **Build System**:
  - Build dependencies, compile, linking, generation and running of output binary or library. (`cargo build` and/or `cargo run`)
  - Test runner (`cargo test`)
  - Project setup (`cargo new`)
  - Documentation generation (`cargo doc`)

**Cargo.toml**

Cargo.toml defines your app config.

The `1.0/hello_world` has 2 tables:

- `[package]`: metadata about the project
- `[[bin]]`: specifies our target (e.g. binary but could be `[lib]` for a library.

#### Using Cargo

- `$ cargo build` && `$ ./target/debug/filename` (manual run step)
- `$ cargo run` (build and run)

### Rust Lang 101

See `book/1.0/basics/basics.rs` for code examples.
See `README_TYPES.md` for type examples.

- **Immutable bindings** by default. Can mutate bindings if `mut` is specified.
- **Strong typing** doesn't automatically turn one type into another
- **Static typing** knows what type elements are before program runs
- **Implicit typing** compilier can often infer binding types
- **Ownership and Borrowing**
  - A variable has ownership of resource they are bound to.
  - When binding is out of scope the resource is freed.
  - Only have 1 binding to a resource at any 1 time.
  - How do we work with our resources? Some options:
    - `Move`
      - We can move values from one binding to another e.g. we re-assign binding (new binding takes ownership) or pass a binding to a function (function takes ownership)
    - `Copy`
      - If the resource is Copy (implements the Copy trait) this allows the resource to be copied instead of moving it to another binding.
    - `Borrow`
      - Borrow cannot last longer than owner scope.
      - Borrow ends when borrower goes out of scope.
      - Only one type of borrow at a time, either:
        - `n Reader Borrows` - give n number of references `(&T)` to a resource.
        - `1 Write Borrow` - give exactly 1 mutable reference `(&mut T)` to a resource. Prevents data races.
- **Traits** - similar to interfaces, they define a set of methods that a type must implement in order to be that trait. Trait methods can then be implmemented by your own types. You can also generate default implementation of traits e.g. using `#[derive(Copy)]` above a type where possible.
- **Lifetimes** - ensures that references do not point to a resource that has been deallocated from memory (dangling pointer). Every reference has a automatic lifetime but sometimes it must be explicitly specified. Needed when using `structs` and `impl` blocks. Lifetimes can be re-used between variables. You can think about lifetimes purely in terms of scopes but naming them allows referring to each variable scope by name.
- **Imports** - packages imported using `use some::module::path`. Some `std` library packages are included automatically e.g. `Vec` is, `LinkedList` is not.

### Sources

- https://www.udemy.com/course/rust-building-reusable-code-with-rust-from-scratch
- Rust Book: https://doc.rust-lang.org/book/
