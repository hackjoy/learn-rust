fn main() {
  let tup: (int, bool) = (41, true);

  match tup {
    (51..53, true) => println!("True and in high range"),
    (40..49, _) => println!("True or false and within low range"),
    (x, y) if x > 53 || x < 51 && y == true => println!("True and out of high range"),
    _ => println!("Nothing matched")
  }
}