# Rust 101

- Rust is a compiled language
- `cargo new project_name`
- `cargo build` gives us a binary executable
- `./target/project_name` or `cargo run` executes it

```rust
use std::cmp::Ordering; // import packages

// fn main runs first
fn main() {
    // VARIABLES
    // *******************************************

    // rust variables are called variable bindings.
    // They must be initialized with a value
    let x = 5;

    // They have some special features like pattern-matching.
    // left hand side of a let expression is a full pattern
    // not just variable binding name. This enables:
    let (y, z) = (1, 2);

    // rust is statically typed and requires variable binding types
    // to be declared upon compile time
    let z: i32 = 5;
    // but rust has type inference so if it can tell what type a
    // variable binding is, you wont need to declare it

    // bindings are immutable by default,
    // compiler will error if you try to mutate.
    // one of the main reasons for this is safety,
    // preventing you from mutating something you may not have meant to.
    // To get a mutable var do:
    let mut x = 10;

    // we interpolate variables with moustaches
    println!("The number {} is a mutable variable", x);

    // BRANCHING
    // *******************************************
    if x == 10 {
        println!("x is ten");
    } else {
        println!("x is something else");
    }

    // Rust is an expression based language with only two
    // kinds of statements
    let y = if x == 10 { 10 } else { 15 };

    // The first kind of statement is a declaration statement
    let z = 5;            // works
    // let is a declaration statement which expects an
    // expression, so the below errors
    let x = (let y = 5); // expected identifier, found keyword `let`

    // the second is an expression statement
    // the semi colon turns any expression into a statement and
    // returns the value () - pronounced unit
    // this errors because rust expects an expression to
    // return a value, instead a unit is returned
    let y: i32 = if x == 5 { 10; } else { 15; };

    // FUNCTIONS
    // *******************************************

    // Functions only return one value and
    // must specify the argument types
    fn print_number(x: i32) {
        println!("x is ()", x);
    }
    // note: no semicolons.
    // functions specify return types if they exist
    fn add_one(x: i32) -> i32 {
        if x < 5 { return x; }
        x + 1
    }

    // DATA TYPES
    // *******************************************

    // TUPLES are a heterogeneous, ordered list
    // of fixed size. It's a form of record type
    let b: (i32, &str) = (1, "hello");

    // You can access the fields in a tuple through
    // a destructuring let. Here's an example:
    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);
    // let is more powerful than just assigning a binding
    // we can put a pattern on the left and match it on the right
    // for multiple assignment

    // One other use of tuples is to return multiple values
    // from a function:
    fn next_two(x: i32) -> (i32, i32) { (x + 1, x + 2) }

    {
        let (x, y) = next_two(5);
        println!("x, y = {}, {}", x, y);
    }
    // Tuples are very simple. so sometimes you need
    // something more powerful

    // STRUCTS are another record type, the difference
    // is they have a name called a field or member
    struct Point {
        x: i32,
        y: i32,
    }

    {
        let mut origin = Point { x: 0, y: 0 }; // origin: Point
        origin.x = 5;
        println!("The origin is at ({}, {})", origin.x, origin.y);
    }


    //  "sum type" / enum type below is in std library
    // an instance of Ordering can be 1 of the three states
    // at any time
    enum Ordering {
        Less,
        Equal,
        Greater,
    }

    // we can use it in code with the folowing
    //  :: refers to a namespace


    fn cmp(a: i32, b: i32) -> Ordering {
        if a < b { Ordering::Less }
        else if a > b { Ordering::Greater }
        else { Ordering::Equal }
    }

    {
        let x = 5;
        let y = 10;

        let ordering = cmp(x, y); // ordering: Ordering

        if ordering == Ordering::Less {
            println!("less");
        } else if ordering == Ordering::Greater {
            println!("greater");
        } else if ordering == Ordering::Equal {
            println!("equal");
        }
    }

    // another example
    enum StringResult {
        StringOK(String),
        ErrorReason(String),
    }

    fn respond(greeting: &str) -> StringResult {
        if greeting == "Hello" {
            StringResult::StringOK("Good morning!".to_string())
        } else {
            StringResult::ErrorReason("I didn't understand you!".to_string())
        }
    }


    // MATCH
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    // Earlier Ordering example written as a match
    // match expressions also allow us to get the values
    // contained in an enum (also known as destructuring) as follows:
    use std::cmp::Ordering;

    fn cmp(a: i32, b: i32) -> Ordering {
        if a < b { Ordering::Less }
        else if a > b { Ordering::Greater }
        else { Ordering::Equal }
    }

    fn main() {
        let x = 5;
        let y = 10;

        match cmp(x, y) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => println!("equal"),
        }
    }

    // LOOPS
    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    // STRINGS
    // Rust has two main types of strings: &str and String.
    // Both UTF-8

    // An &str is statically allocated, meaning that it's
    // saved inside our compiled program, and exists for the
    // entire duration it runs. The string binding is a reference
    // to this statically allocated string. String slices have
    // a fixed size, and cannot be mutated.
    let string = "Hello there."; // string: &str

    // A String is an in-memory string
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    // You can get a &str view into a String with the as_slice() method:

    fn takes_slice(slice: &str) {
        println!("Got: {}", slice);
    }

    {
        let s = "Hello".to_string();
        takes_slice(s.as_slice());
    }
    // To compare a String to a constant string, prefer as_slice()...

    fn compare(string: String) {
        if string.as_slice() == "Hello" {
            println!("yes");
        }
    }

    // Just remember that Strings allocate memory and control
    // their data, while &strs are a reference to another string,

    // ARRAYS
    // Arrays are fixed size and have type [T; N].

    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // mut m: [i32; 3]

    let a = [0; 20]; // a: [i32; 20]   - each element initialized to 0
    println!("a has {} elements", a.len());
    println!("first element is {}", a[0]);
    for e in a.iter() {
        println!("{}", e);
    }

    // VECTOR
    // A dynamic array with type Vec<T>, init with a macro vec!
    // arguments to macros can be wrapped in [] or () depending on situation
    let v = vec![1, 2, 3]; // v: Vec<i32>
    let mut nums = vec![1, 2, 3]; // mut nums: Vec<i32>
    nums.push(4);
    println!("The length of nums is now {}", nums.len()); // Prints 4

    // A slice is a reference to (or "view" into) an array. memory efficient
    // without copying - slices can be mutable or not
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4]; // A slice of a: just the elements 1, 2, and 3

    for e in middle.iter() {
        println!("{}", e); // Prints 1, 2, 3
    }

    // You can also take a slice of a vector, String, or &str,
    // because they are backed by arrays. Slices have type &[T],
    // which we'll talk about when we cover generics.

    // STANDARD INPUT
    use std::old_io;

    {
        println!("Type something!");

        // here, we'll show the types at each step
        let input = old_io::stdin() // std::old_io::stdio::StdinReader
                      .read_line() // IoResult<String>
                      .ok() // Option<String>
                      .expect("Failed to read line"); // String

        println!("{}", input);
    }
}
```
