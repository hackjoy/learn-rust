# Deref

- Crucial component of smart pointers
- A trait that implements the behaviour of the dereference operator `*`.
- If you implement `Deref` in a way that allows a smart pointer (`Box<T>`) to be treated like a regular reference `&T` you can write code that operates on both references and smart pointers.

- When calling `deref` (`*`) on a regular reference, we follow the pointer (get the value) from the memory address. This works the same for Box. It doesn't work by default for a custom type, you have to implement Deref for it to work.

````rust
// Regular derer
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref regular reference (&)
}```

```rust
use std::ops::Deref;

fn main() {
    struct MyBox<T>(T); // tuple struct with one element of type T.
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T; // Target is an associated type used by Deref trait, slightly different way of declaring a generic parameter

        fn deref(&self) -> &Self::Target {
            &self.0 // return regular ref to value in MyBox tuple
        }
    }

    let x = 5;
    let y = &x;
    let z = Box::new(5);

    let a = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref regular reference (&)
    assert_eq!(5, *z); // Deref Box<>, same as regular reference
    assert_eq!(5, *a);  // Rust actually runs *(a.deref())
}
````

### Deref coercion

- Convert `&T` (which implements `Deref`) -> `&AnotherT`
- A convenience Rust probides on arguments to functions

```rust
// Function takes a reference to string slice hello("ExampleStringSlice")
fn hello(name: &str) {
    println!("Hello, {name}!");
}
fn main() {
    let m = MyBox::new(String::from("Rust"));
    /*
     - Rust analyses the types used at compile time and use Deref::deref as many times as necessary to get a reference to match the hello function parameter’s type.
     - &m = &MyBox<String> (deref)-> &String (deref)-> &str
     - Otherwise we'd have to do it ourselves: hello(&(*m)[..]);
    */
    hello(&m);
}
```

`DerefMut` can be used in the same way for mutable references.
