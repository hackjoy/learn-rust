fn main() {
  for num in range(0, 10) {
    spawn(proc() {
      let greeting_message = "Hello";
      println!("{} I am process number {}", greeting_message, num);
    });
  }
}