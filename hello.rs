fn main() {
  let my_greeting = "Hello!";           // declare immutable variable. Rust infers type. Use underscores.
  let mut count = 0;                    // declare mutable variable. Rust can infer the type.

  // static MONSTER_FACTOR: f64 = 57.8; // you can specify type if you want.
  // let monster_size: int = 50;        // you can specify type if you want.
  // type MyType = int;                 // Types are CamelCase apart from primitives

  // conditions dont need to be in (), bodies must be wrapped in {}
  while count < 10 {
      println!("{}", my_greeting);
      count += 1;
  }
}

