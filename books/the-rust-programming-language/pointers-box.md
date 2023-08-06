## Summary

- Box<T> stores data on heap rather than stack
- All that remains on stack is the Box pointer itself.
- No performance overhead as Box just provides indirection and heap allocation.
- Other smart pointers have special capabilites along with performance overhead.
- `Box<T>` is smart because it
  - implements `Deref` which allows `Box<T>` values to be treated like references.
  - implements `Drop` which means that heap data is deallocated when `Box<T>` goes out of scope

### When to use:

- When you need indirection and/or heap allocation.
- Core use cases:
  - 1.  You have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
  - 2.  You have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so (can be)
  - 3.  You want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### How to use:

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b); // b = 5
    // The value can be used as if let b = 5 same as if declared on stack.
}
```

### Example 1. Size of T cannot be known at compile time

```rust
/*
- When creating recursive type List, Rust compiler needs to know how much data to allocate for a List on the stack, so checks each variant but on each check of Cons, it must again check for List and the cycle repeats infintely, Rust does not know how much to allocate for the List.
- Box pointers have a known size on the stack and can point to data of unknown size on the heap. We can use a Box pointer to break the recursive chain.
*/
fn main() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};

    let _list =
        Cons(1, Box::new(
            Cons(2, Box::new(
                Cons(3, Box::new(Nil))
        ))
    ));
 }
```
