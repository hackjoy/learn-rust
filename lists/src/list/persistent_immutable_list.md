Common list operations:

```
list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D
```

Desired memory layout

```
list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+
```

Who frees `B`? Ownership of it is shared, so `Box` type cannot work.
 
Garbage Collection (GC) solves this in most languages (typically *tracing GC*). 

Rust only has reference counting if we want this type of behaviour - a simple form of GC. 

This is often less overhead than a tracing GC for many use cases but will cause deallocation issues if you create cycles.

A good use of an immutable list is to share data across threads. But this requires atomic adjustments to the reference counts - the data could be freed too soon. We can use `Arc` instead of `Rc` to get atomic reference counting and thread safety.

Rust models thread safety with two traits Send and Sync.

Type is `Send` if it's save to move to another thread
Type is `Sync` if it's safe to share between multiple threads.
These types ensure no data races but not race conditions. They are automatically derived if your entity is entirely composed of Send/Sync types. Same concept for Copy trait (can only be Copy if all your types are Copy). 

Most types are Send because they totally own their data. Most types are Sync because the only way to share data across threads is to put them behind a shared reference (therefore immutable).

Types that have interior mutability do not have this. 

So far we've only seen inherited mutability (AKA external mutability): the mutability of a value is inherited from the mutability of its container. That is, you can't just randomly mutate some field of a non-mutable value because you feel like it.

Interior mutability types violate this: they let you mutate through a shared reference. There are two major classes of interior mutability: cells, which only work in a single-threaded context; and locks, which work in a multi-threaded context. For obvious reasons, cells are cheaper when you can use them. There's also atomics, which are primitives that act like a lock.

So what does all of this have to do with Rc and Arc? Well, they both use interior mutability for their reference count. Worse, this reference count is shared between every instance! Rc just uses a cell, which means it's not thread safe. Arc uses an atomic, which means it is thread safe. Of course, you can't magically make a type thread safe by putting it in Arc. Arc can only derive thread-safety like any other type.

Atomic memory models or non-derived Send implementations are complex but as a high level consumer we could use `Arc` and be done with it.