# Weak<T>

## Overcoming Circular References / Memory Leaks with Rc<T> and RefCell<T>

- It is possible to incur memory leaks in Rust (in a safe way) when using `Rc` and `RefCel` - where a reference count never drops to 0.
- We can use `Weak<T>` smart pointers to help overcome this.

### Example: Cycle with Rc<T> and RefCell<T> that will cause stack overflow

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // Allow Cons element to be changed
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // modify tail of a to point to b, creating a cycle
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncommenting below would encounter the cycle and cause a stack overflow
    // println!("a next item = {:?}", a.tail());
}
```

- If you have `RefCell<T>` values that contain `Rc<T>` values, you must manually ensure no cycles are created.

  - This applies to similar nested types with interior mutability and reference counting

- One solution for avoiding reference cycles is reorganizing your data structures so that _some_ references express ownership and _some_ references don’t.

  - You can then have cycles made up of some ownership relationships and some non-ownership relationships, and only the ownership relationships affect whether or not a value can be dropped.

- Another solution is to use `Weak<T>` pointers.
  - `Rc::clone` increases the `strong_count` by 1 of a `Rc<T>`. The variable cloning then shares ownership.
  - `Rc::downgrade` only increases the `weak_count` and returns a `Weak<T>` - the variable using the downgrade does not get shared ownership, does not increase the `strong_count`.
  - At any time the `Weak<T>` may have been dropped so when accessing the reference we need to call `upgrade` which returns an `Option<Rc<T>>` (`Some(Rc<T>)` or `None`)

### Example: Tree with Child/Parent (Cyclical) Relationships with No Memory Leaks

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // Node does not own parent but can refer to it via Weak reference
    // Weak reference enables parent to continue existing when Node dropped from memory
    // Weak reference enables multiple references by Nodes to the same parent. The RefCel means that Node owns the parent reference but not the underlying Parent - we must ensure that we follow borrow rules in use of the RefCell and be prepared for the parent not to exist due to the Weak reference
    parent: RefCell<Weak<Node>>,
    // Node owns it's children, When we drop Node, children will no longer exist.
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // parent = None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // parent = Some(Node)
}
```
