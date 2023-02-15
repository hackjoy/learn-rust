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




