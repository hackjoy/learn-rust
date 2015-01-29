// fizzbuzz in rust:
fn main() {
  let mut num = 0;
  while num < 20 {
    if num % 3 == 0 && num % 5 == 0 {
      println!("FizzBuzz")
    } else if num % 3 == 0 {
      println!("Fizz")
    } else if num % 5 == 0 {
      println!("Buzz")
    } else {
      println!("{}", num)
    }
    num += 1
  }
}
