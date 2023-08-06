# Drop

- Crucial component of smart pointers
- Drop trait defines what happens when value is about to go out of scope - it is called automatically. Some languages require manually freeing memory resources
- Before out of scope can release files or network connections etc
- Variables are dropped in reverse order of creation.

```rust
struct CustomSmartPointer {
    data: String,
}

// Drop trait automatically in scope as part of prelude
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // drop(c) // - We could call drop early by using the prelude's  drop e.g. may do this if we had a smart pointer that managed locks for example
    println!("CustomSmartPointers created.");
}
```
