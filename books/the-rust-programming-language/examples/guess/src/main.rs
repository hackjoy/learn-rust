use std::old_io;
use std::rand;
use std::cmp::Ordering;

fn main() {
    // generate random number from 1-100
    // use unsigned int which can only be a positive num
    let secret_number = (rand::random::<u32>() % 100) + 1;

    println!("Guess our secret number from 1-100!");
    loop {
        println!("Please input your guess.\n");

        let input = old_io::stdin()
                      .read_line()
                      .ok()
                      .expect("Failed to read line.");
        let input_number: Option<u32> = input.trim().parse();
        let num = match input_number {
            Some(num) => num,
            None => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed {}...", num);

        match cmp(num, secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
                  println!("You win!");
                  return;
              }
        }
    }
}

fn cmp(a: u32, b: u32) -> Ordering {
  if a < b { Ordering::Less }
  else if a > b { Ordering::Greater }
  else { Ordering::Equal }
}
