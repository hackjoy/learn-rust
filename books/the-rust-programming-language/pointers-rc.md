# Rc<T>

- Usually ownership is 1 variable -> 1 value but there are cases where that is not possible.
- `Rc<T>` is a reference counted pointer type that allows multiple owners of a single value - e.g. graph structure where multiple edges point to same node.
- `Rc<T>` type keeps count of number of references to a value. If 0 then the value is no longer in use and we can drop().
- Used when allocating data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last.
- Only used in single threaded programs and immutable cases
- If we want mutability with multiple owners we can use `RefCell<T>`

## Usage:

- To create 2 (b,c) lists that both pointed to the same list (a)
- We can't do this with `Box<T>` and the original Cons definition in `pointers-box.md` - we would get a use of moved value error.
- We could make `Cons` work based on references (&) rather than owned data but that would require defining lifetime parameters which in this case would mean each value lives as long as the enture list.
- Alternatively we can use `Rc<T>` to allow multiple ownership.

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;


fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = Cons(3, Rc::clone(&a)); // Cloning Rc<List> using the reference increases the reference count of a
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = Cons(4, Rc::clone(&a)); // NOT a deep clone of a, only increments the ref count which is fast.
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2 (c went out of scope)
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```
