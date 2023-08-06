# RefCell<T>

- Interior mutability is a design pattern in Rust that allows mutation of data even when there are immutable references to that data.
- This pattern uses unsafe (programmer checked) code inside a data structure to override normal borrowing rules.
- However we must manually ensure that at runtime we follow the borrowing rules. (1 write reference or n read references) - otherwise the program will panic! if broken.
- To create normal references we use `&T` and `&mut T`
- To create compile safe references with a `RefCel<T>` we use
  - `.borrow()` which gives the smart pointer `Ref<T>`
  - `.borrow_mut()` which gives us the smart pointer `RefMut<T>`
  - Both types implement `Deref` so can be used like regular references.
- `RefCel<T>` keeps count of the number of `Ref<T>` and `RefMut<T>` that are active.
- _The programmer_ must ensure only 1 `RefMut<T>` or n `Ref<T>` exist - it will not cause a compiler error but a runtime panic!

- `RefCell<T>` has single ownership of data it holds, used in single threaded programs.
- Compile time checks of ownership are best but they are conservative to guarantee correctness - there are cases where memory safe scenarios with only runtime checks are possible. Problems like the Halting problem are impossible to detect through compile time analysis.
- `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee by itself. We can do things like mutating values in a context where only immutable values are allowed.

### Example: Mock Objects

- test double - using 1 type to take the place of another during testing.
- Rust doesn't have objects in the way other languages do and no built in provisions.
- But we can provide the same behaviour via a struct.
- Example code:
  - A library that tracks a value against a maximum value
  - Sends messages based on value is to maximum
  - User implements the send trait so they can log, send SMS, send email etc
  - Use case: API quota tracking

```rust
// Define trait with single method send() that takes reference to self and the string slice msg.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // The method we want to test - nothing returned, we want to check send() called correctly
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>, // wrap in RefCell so we can mutate wihtin send()
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // immutable borrow of self (&self) as per Messenger Trait
        fn send(&self, message: &str) {
            // however we mutably borrow the RefCell
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

#### Choosing between `Box<T>`, `Rc<T>`, or `RefCell<T>`

**Ownership**

- `Rc<T>` - allows multiple owners of the same data;
- `Box<T>` and `RefCell<T>` - single owners only.

**Immutability**

- `Box<T>` - immutable or mutable borrows checked at compile time
- `Rc<T>` - immutable borrows checked at compile time
- `RefCell<T>` immutable or mutable borrows checked at runtime.
  - Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable and appears immutable to our code.

#### Combining `Rc<T>` and `RefCell<T>`

- If you have a `Rc<T>` that holds a `RefCell<T>`, you can have multiple owners and mutability.
- See our updated `Cons` example.
  - Both `b` and `c` own `a`.
  - `a` owns `value`.
  - `value` can be mutated by a mutable borrow.
  - No mutable variables are declated, the RefCell is allowing us to do this and ensure that we do not violate borrowing rules. This is outward immutability and interior mutability.
  - Sometimes we need to trade off speed for this flexiblity with our data structures.
  - Remember that `RefCell` is single thread only. `Mutex` is multi-thread.

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); // value passed as a Rc so that value is

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a = {:?}", a); // a = Cons(RefCell { value: 15 }, Nil)
    println!("b = {:?}", b); // b = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    println!("c = {:?}", c); // c = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))

}
```
