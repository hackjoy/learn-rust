// STACK vs HEAP
// http://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap

/* 
   ### POINTERS 101
   - Pointers are variables that contain (point to) a memory address. The memory address contains data of type T
   - Reference pointers are references (&T or &mut T) that simply hold the memory address.
     - Default to using reference pointers as stack has lower overhead vs heap
   - Smart pointers are data structures that act like a pointer but also contain metadata and special capabilities.
     -  String and Vec<T> can be considered smart pointers as they own memory and allow you to manipulate it. 
     - e.g. String stores its capacity as metadata and has ability to ensure its data is valid UTF-8.
   - Smart pointers usually implemented using structs. Unlike basic structs, smart pointers implement the Deref and Drop traits. 
     - Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers. 
     - Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

   ### REFERENCE POINTERS - BORROWING DATA

   &T	     Reference 
   - Borrow allowing 1+ references to access T (read-only)
   - Data referenced may be on stack or heap.
   - Common pointer. No special capabilities, low overhead.
   
   &mut T	 Mutable Reference (Reference pointer)
   - Borrow allowing 1 single reference to access T (read + write).
   - Data referenced may be on stack or heap.
   - Common pointer. No special capabilities, low overhead.

   ### SMART POINTERS - POINTER USUALLY OWNS THE DATA

   Box<T>    Box 
   - Heap allocated T (T moved to heap if not already) - only pointer itself remains on stack. 
   - 1 owner to read/write T.
   - Simplest of the smart pointers. No performance overhead.
   - Use when:
    - You have a type whose size is unknown at compile time and you want to use a value of that type in a context that requires an exact size
    - You have a large amount of data and you want to transfer ownership and ensure data is not copied
    - You want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type 

   Rc<T>	 RC pointer	   
   - Heap allocated T
   - Allows multiple owners each with read-only access
   
   Arc<T>	 Arc pointer 
   - Heap allocated T
   - Allows multiple owners each with multithread-safe read-only access 

   RefCell<T> (for non Copy types) and Cell<T> (for Copy types)
   - Allows dynamic (runtime) borrow checks. Must conform to rust borrowing rules (n readers or 1 writer reference).
   - To get a readle smart pointer (Ref<T>) call borrow() on the RefCell<T>
   - To get a writable smart pointer (RefMut<T>) call borrow_mut() on the RefCell<T>

   Mutex<T>  mutex pointer
   - Allows read-write across threads using locks to ensure 1 thread has access at a time
   - Useful in concurrency to prevent data races as everything is synchronised by the lock.

   *const T	 Raw pointer
   - Unsafe read access to T

   *mut T	 Mutable raw pointer
   - Unsafe read and write access to T

*/

// ### REFERENCE POINTER EXAMPLES

let x: i32 = 5;  // 5 allocated memory in stack
let y: &i32 = &x; // &x is rust reference that points to a value. Y has borrowed X via immutable reference pointer.
println!("The value of y is {}", y);  // 0x3e3243
println!("Deference the pointer y, get the value stored at the address {}", *y);  // 5
println!("The value of x is {}", x);  // 5
println!("The memory location of x is {:p}", &x);  // 0x3e3243
// References immutable by default. 

/*
- Any time you have a structure that can change in size, you need a pointer.
- You can't tell at compile time how much memory to allocate, so pointers point at the memory where it will be allocated at runtime
- Rust pointers are slightly more complicated than C but this gives many protections against common errors when using them.
*/ 

// Function which takes a reference as parameter
fn succ(x: &i32) -> i32 { *x + 1 } // *x dereferences/accesses the value at memory location.

// Borrowing within a function 
fn this_borrow_works() { 
    let mut x = 5;
    if x < 10 {
        let y = &x; // Borrow reference to x
        println!("We have y: {}", y);
        return; // y goes out of scope
    }
    x -= 1; // y scope ended so no longer borrowed and x once again owner
}
fn this_borrow_does_not_work() {
    let mut x = 5;
    if x < 10 {
        let y = &x;
        x -= 1; // Compiler error - x borrowed by y and y not gone out of scope yet
        println!("This won't print: {}", y);
        return;
    }
    x -= 1;
}

// Best practices
// 

// Use references when you want to use a pointer. y = &x
// this way you borrow the ownership of the memory address,
fn succ(x: &i32) -> i32 { *x + 1 }
// rather than taking ownership like this:
fn succ(x: Box<i32>) -> i32 { *x + 1 }
// also using a &reference as a parameter allows you to accept a wide variety of
// other pointers

// ### BOXED POINTER
// Note that Box is used with more complex types than i32

{
    let x = Box::new(5);
    // do stuff with x
} // x automatically destructed and memory (deallocated) here

// Using boxes and references together is very common:
fn add_one(x: &i32) -> i32 {
    *x + 1
}
fn main() {
    let x = Box::new(5);
    // Here we borrow the value inside the box.
    println!("{}", add_one(&*x)); // dereference Box (get i32) and borrow the i32 (get a reference pointer)
}
// Rust knows x is being borrowed by the add_one() function and allows as read only.

// Can also borrow mutable variable simultaneously
fn increment(x: &mut i32) {
    *x += 1;
}
fn main() {
    let mut x = Box::new(5); // x must be declared as "mut"
    increment(&mut *x);
    increment(&mut *x);
    println!("{}", x);
}
