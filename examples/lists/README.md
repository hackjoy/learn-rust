### [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)

#### Notes

Some use cases for a Linked List:
- Lots of splitting or merging of big lists
- Lock-free concurrency
- Kernel/embedded thing and want to use an intrusive list.
- You're using a pure functional language and the limited semantics and absence of mutation makes linked lists easier to work with.

But in Rust you can often avoid Linked Lists and use:
- `Vec` (array stack) 99% of the time
- `VecDeque` (array deque) the other 1% 
  
Why? Less frequent allocation, lower memory overhead, true random access, and cache locality.

Linked Lists are as niche as trie structures. Useful only in exceptional circumstances. Arguments for this [here](https://rust-unofficial.github.io/too-many-lists/#performance-doesnt-always-matter). A Linked HashMap may be a better choice if needing a linked list.

Approximate definition of a linked list is pieces of data on the heap that point to each other in sequence.

A function definition:
`List a = Empty | Elem a (List a)` 

"A List is either Empty or an Element followed by a List"

We could implement as follows but this has some drawbacks:
```
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
```
`[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)`
- Last Node is not actually a Node, first node is not heap allocated.

#### Option
- A type that represents the presence or absence of a value (Some(T) or None)
- type has no overhead for &, &mut, Box, Rc, Arc, Vec + more due to null pointer optimization.

#### Enum 
- declares a type that can be 1 of several values. 

#### Struct
- Declares a type that can contain many values at once.

A better approach 

```
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
```

Now tail of a list never allocates extra empty nodes, enum is in null-pointer-optimized form, all elements are uniformly allocated 

The above approach keeps the public types to a minimum.

#### Ownership

```
impl Something {
  pub fn foo(self, elem: i32) -> SomethingElse {
    // Code
  }
}
```
- First arg can be: `self` (Value), `&mut self` (mutable ref), `&self` (shared ref)
  - self (Value): True ownership, can move, mutate, loan via ref. When passing by Value the new location (foo) owns the value, the old location can no longer access it. Therefore we don't typically want to pass the value as the calling code loses access to it.
  -  `&mut self` (Mutable ref): temporary exclusive access to value you do not own allowing mutation. You cannot move the value.
  - `&self` (Shared ref): temporary shared access to value you do not own. You cannot move/mutate the value. Can bypass immutability in some cases so called shared references rather than immutable

#### Trait
- Similar to an interface - functionality that a type has and can be shared with other types. The type must provide a specific implementation for the trait - it just conforms to the function signatures within the trait.


#### Drop
- A trait with a single method drop() that is called when an object goes out of scope.
- Not required to implement for types that implement Drop

### Lifetime

- Lifetimes prevent a program holding a pointer to something out of scope or something that got mutated away.
- A lifetime is the name of the region (~block/scope) of code somewhere in a program. When a reference is tagged with a lifetime, we are declaring it must be valid for that entire region.
- Different things place requirements on how long a reference must and can be valid for. The entire lifetime system is a constraint-solving system that minimises the region of each reference. If it successfully finds a set of lifetimes that satisfies all the constraints, your program compiles! Otherwise you get an error back saying that something didn't live long enough.
- Within a function body the compiler has full information to infer constraints to find the minimum lifetimes.
- At the type and API-level, the compiler doesn't have all the information. It requires you to tell it about the relationship between different lifetimes 
- When using references - some lifetime cases are so common that Rust will automatically pick the lifetimes (a.k.a. lifetime elision)

Examples:

```rust
// Only one reference in input, so the output must be derived from that input
fn foo(&A) -> &B; // sugar for:
fn foo<'a>(&'a A) -> &'a B;

// Many inputs, assume they're all independent
fn foo(&A, &B, &C); // sugar for:
fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

// Methods, assume all output lifetimes are derived from `self`
fn foo(&self, &B, &C) -> &D; // sugar for:
fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;
```


```rust
fn foo<'a>(&'a A) -> &'a B
```
Above means the input must live at least as long as the output. If the output is kept around for a long time, it expands the region that the input must be valid for. 
Once output B is no longer in use, the compiler knows it can free A