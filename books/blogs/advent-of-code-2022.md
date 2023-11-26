## Advent of Code 2022

#### Source: https://fasterthanli.me/series/advent-of-code-2022

1. Parsing a file, find the maximum sum of consecutive lines of values. Each set of values is separated by a line break.

- `let f = std::fs::read_to_string("some/path.txt)` returns an `enum` called `Result` which has 2 generic type parameters: `Result<T, E>`. When successful it will return `Ok(v: T)` or `Err(e: E)`
- As `f` is a `Result` to check the actual variant and get the value `v` or error `e`, we need to extract it from `f`
- We can use:
  - `match f { Ok(v) => handle_v, Err(e) => handle_e }`
  - `let v = f.unwrap()` which will extract v from the result if it's `Ok` and `panic!()` (terminate the program) with the error if not.
  - `let v = f.expect("could not parse some/path")` - which pulls out the value or adds additional context to a `panic!()` error.
- If our `read_to_string` call was extracted into a function `read_input()` and we didn't want an error to cause a panic in `read_input()` we wouldn't be able to use `unwrap()` or `expect()` there, we'd have to propagate the error back to `main` so that `unwrap` can be called in main. We could either:
  - Return a `Result` from `read_input()` which would allow propagation of the error without e.g. `fn read_input() -> Result<String, std::io::Error>` but using `unwrap()` in main would mean we lose context of the specific error.
  - We could implement our own error `fn read_input() -> Result<String, FileReaderError>` with additional context but that would require additional work (e.g. implmentation of Debug trait or use of the debug derive macro) for a very common scenario. It would have to conform the signature for `unwrap()` as per the std::result type:
    ```rust
    impl<T, E> Result<T, E> {
        pub fn unwrap(self) -> T
        where
            E: fmt::Debug, // fmt::Debug is a 'trait' bound applied to E
        {
        // Implementation
        }
    }
    ```
  - We could use a package `fs_error` and then `fs_error::read_to_string()` instead of `std::fs::read_to_string()` for better errors.
  - We could use a package `color-eyre` that implements a `Result` type that retains context of where the error was initially built, with optionally additional context. You could extend this example further to make main return a color_eyre::Result as the result of the program (as opposed to a panic! with the error)
    ```rust
    use color_eyre::eyre::Context;
    fn main() {
        color_eyre::install().unwrap();
        let input = read_input().unwrap();
    }
    fn read_input() -> color_eyre::Result<String> {
        // The ? operator means extract the Ok(value) or return ("propagate") the Err(e) to the caller
        let input = std::fs::read_to_string("src/input.txt").wrap_err("reading src/input.txt")?;
        Ok(input)
    }
    ```
- Moving to solve the problem - we can read file at compile time with `include_str!("path/to/file")` so it will be part of the executable.
- `Option<T>` is an enum similar to `Result<T, E>` except that the variants of an option are: `Some(T)` or `None`
- We can destructure an option just like a result to obtain the variant values - using `match` or `if let`
- Straightforward solution:

```rust
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut max = 0;
    for group in include_str!("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        let mut sum = 0;
        for line in group.lines() { // lines() returns an iterator, 'for' invokes next()
            let value = line.parse::<u64>()?;
            sum += value;
        }
        if sum > max {
            max = sum;
        }
    }
    println!("{}", max);
    Ok(())
}
```

- Another solution with map, collect

```rust
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let lines = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();  // convert iterator into a Vec or type _ (in this case Option<u64>[])
    let max = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max();
    println!("{}", max);
    Ok(())
}
```

- Advanced iterator use with itertools

```rust
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max();
    println!("{max:?}");

    Ok(())
}
```

- Extending this to find the total of the 3 largest cumulative sums of values

```rust
use itertools::{FoldWhile, Itertools};
use std::cmp::Reverse;
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let answer = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok()) // separated lines by `None`
        .batching(|it| {
            it.fold_while(None, |acc: Option<u64>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                None => FoldWhile::Done(acc), // that's the group separator, `fold_while` is done
            })
            // this returns `Some(total)` if we found a group, `None` if we're
            // at the end, to let `batching` end.
            .into_inner()
        })
        .map(Reverse) // this turns `k_smallest` into `k_largest`
        .k_smallest(3)
        .map(|x| x.0) // strip the `Reverse` so we can sum
        .sum::<u64>();
    println!("{answer:?}");
    Ok(())
}
```

- Then build binary for release `cargo build --release`
- Time `hyperfine ./target/release/program_name`
