fn main() {
    // rust variables are called variable bindings.
    // They must be initialized with a value
    let x = 5;

    // They have some special features like pattern-matching
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
    let mut x = 10

    // we interpolate variables with moustaches
    println!("The number {} is a mutable variable", x);


    // Branching
    if x == 10 {
        println!("x is ten");
    } else {
        println!("x is something else");
    }

    // Rust is an expression based language. Two kinds of statements and
    // everything else is an expression
    let y = if x == 10 { 10 } else { 15 };

    // The first statement is a declaration statement
    // the below cannot be used as an expression
    let z = 5            // works
    let x = (let y = 5); // expected identifier, found keyword `let`


}