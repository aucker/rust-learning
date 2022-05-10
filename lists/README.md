## Just as [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) describes, The author of this tutorial hates *Linked Lists*, they are terrible data structures. Whereas there are several great use cases for a linked list:

* You want to do a lot of splitting or merging of big lists. A lot
* You're doing some awesome lock-free concurrent thing.
* You're writing a kernel/embedded thing and want to use an intrusive list.
* You're using a pure functional language and the limited semantics and absence of mutation makes linked lists easier to work with
* ...and more!

But all of these cases are *super rare* for anyone writing a Rust program. 99% of the time you should just use a Vec(array stack), and 99% of the other 1% of the time you should be using a VecDeque(array deque). These are blatantly superior data structures for most workloads due to less frequent allocation, lower memory overhead, true random access, and cache locality.