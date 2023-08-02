**Numeric Types:**

- Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (platform dependent int)
- Unsigned Integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (platform dependent uint)
- Floating-point: `f32`, `f64`
- Character: `char` (4 byte Unicode scalar values)
  **Collection Types:**
- Arrays:
  - `let arr: [i32; 5] = [1, 2, 3, 4, 5];`
  - `arr[1] // returns 2`
  - Fixed size. Stack allocated.
- Tuples:
  - `let tuple: (i32, f64, char) = (42, 3.14, 'A');`
  - `tuple.1 // returns 3.14`
  - Fixed size. Stack allocated.
- Vec<T>: - Dynamiccally sized/growable vector, containing elements of type T. - Heap allocated.
  **Boolean Type:**
- `bool`: Represents `true` or `false` values.
  **Unit Type:**
- `()`: Represents an empty tuple or `unit` - used when no meaningful value needs to be returned.
  **Slices:**
- `&[T]`: Reference to a contiguous sequence of elements of type `T`.
  **Pointers:**
- Raw Pointers: `*const T` and `*mut T`, used for unsafe code.
- Function Pointers: `fn()`, used to store the address of a function.
  **References:**
- `&T`: A reference to a value of type `T`.
- `&mut T`: A mutable reference to a value of type `T`.
  **Strings:**
- `str`: A UTF-8 encoded string slice.
- `String`: A growable, heap allocated UTF-8 encoded string.
  **Option and Result:**
- `Option<T>`: Represents an optional value, either `Some(T)` or `None`.
- `Result<T, E>`: Represents the result of an operation, either `Ok(T)` or `Err(E)`.
  **Struct:**
- Custom data type that combines multiple values of different types into a single named entity.
- ```rust
      struct Person {
          name: String,
          age: u32,
      }
  ```
