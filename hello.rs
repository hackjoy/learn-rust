fn main() {

  /* 
     VARIABLES
  */

  // let voodoo: int;                   // declare immutable variable - unused
  let my_greeting = "Hello!";           // declare immutable variable. Rust infers type. Use underscores.
  let mut count: int = 1;               // declare mutable variable. Rust can infer the type.

  // static MONSTER_FACTOR: f64 = 57.8; // if it could be unclear what the type is, explicity define it
  // let monster_size: int = 50;        // you can specify type if you want.
  // type MyType = int;                 // Types are CamelCase apart from primitives

  // let tup = (4, 5.0, false, "hello");  // declare tuple with mixed types

  /* 
     CONDITIONALS & LOOPS
  */
  if true != false {
    println!("All is well.");
  }
  else if true == true {
    println!("Great.");
  }
  else {
    println!("Things are not what they seem..");
  }
  // conditions dont need to be in (), bodies must be wrapped in {}

  while count < 10 {
      println!("{}", my_greeting);
      count += 1;
  }

  let x: int = 4;
  match x {
    0 => { ; } // Do nothing
    4 => { println!("Still the same"); } 
    _ => { println!("Huh?"); } // Matches every integer value
  }

}