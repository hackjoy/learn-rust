fn main() {
  fn foo(x: int) -> int {
    return plus_two(x);
  }

  fn plus_two(x: int) -> int {
    return x + 2   
  }

  foo(2);
}