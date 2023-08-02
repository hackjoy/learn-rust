// STACK vs HEAP
// http://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap

// POINTERS

// Type  	 Name           	   Summary
// &T	     Reference	           Allows one or more references to read T
// &mut T	 Mutable Reference	   Allows a single reference to read and write T
// Box<T>    Box	               Heap allocated T with a single owner that may read and write T.
// Rc<T>	 "arr cee" pointer	   Heap allocated T, read-only for any number of consumers
// Arc<T>	 Arc pointer	       Same as above, but safe sharing across threads
// *const T	 Raw pointer	       Unsafe read access to T
// *mut T	 Mutable raw pointer   Unsafe read and write access to T

// Pointers point to a memory location

// SIMPLE POINTER - Stack variable example
let x = 5;  // memory allocated in stack
let y = &x; // &x is rust reference, y is a simple pointer, an address that points to a value

println!("The value of y is {}", x);  // // 0x3e3243
println!("Deference the pointer y, get the value stored at the address {}", *y);  // 5
println!("The value of x is {}", x);  // 5
println!("The memory location of x is {:p}", &x);  // 0x3e3243

// Any time you have a structure that can change in size, you need a pointer.
// You can't tell at compile time how much memory to allocate, so you've gotta
// use a pointer to point at the memory where it will be allocated, and deal with it at run time

// Rust pointers are slightly more complicated than C for example
// but this gives many protections against common errors when using them

// references immutable by default

// Function which takes a reference
// - e.g. if y = &x, pass in y      if x = 5, pass in &x
fn succ(x: &i32) -> i32 { *x + 1 }

// Borrowing in Rust
// We borrow a pointer to x inside of the if.
// The compiler, however, is able to determine that
// that pointer will go out of scope without x being mutated,
// and therefore, lets us pass.
fn main() {
    let mut x = 5;

    if x < 10 {
        let y = &x;

        println!("Oh no: {}", y);
        return;
    }

    x -= 1;
}

// You couldnt do this:
fn main() {
    let mut x = 5;

    if x < 10 {
        let y = &x;

        x -= 1; // Compiler blows up because x is borrowed by y!

        println!("Oh no: {}", y);
        return;
    }

    x -= 1;
}


// Best practices

// Prefer stack over Heap memory allocation
// Therefore references are the default pointer type you should use

// Use references when you want to use a pointer. y = &x
// this way you borrow the ownership of the memory address,
fn succ(x: &i32) -> i32 { *x + 1 }
// rather than taking ownership like this:
fn succ(x: Box<i32>) -> i32 { *x + 1 }
// also using a &reference as a parameter allows you to accept a wide variety of
// other pointers

// BOXED POINTER - heap allocated, unallocated when out of scope,
{
    let x = Box::new(5);
    // stuff happens
} // x is automatically destructed and its memory is free'd (deallocated) here

// An affine type - Not garbage collected, compiler at compile time knows when in and out of scope
// This prevents many errors in allocating / freeing memory

// Using boxes and references together is very common. For example:

fn add_one(x: &i32) -> i32 {
    *x + 1
}
fn main() {
    let x = Box::new(5);
    println!("{}", add_one(&*x));
}
// In this case, Rust knows that x is being borrowed by the add_one() function,
// and since it's only reading the value, allows it.

// Can also borrow mutable variable simultaneously
fn increment(x: &mut i32) {
    *x += 1;
}
fn main() {
    // If variable x is not "mut", this will not compile
    let mut x = Box::new(5);
    increment(&mut x);
    increment(&mut x);
    println!("{}", x);
}
