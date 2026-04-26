## Rust Basics
4/25/2025

- fn name(arg: Type) -> Ret { ... } last expression w/o ; returns.
- Mutability is opt-in.
- Copy is safe iff the value's bits are the value i.e. they are in the stack. If there's a heap pointer, everything is a unique pointer.
- Borrow with let borrower = &owner. Borrower points to the stack header of the owner.
- Borrowing rules:
    - Reference can be copied because its a memory address that fits fully in stack.
    - At any moment, for any value, you only have either any number of read-only references, or exactly one mutable reference, but nether both at the same time.
    - Plus, every reference must be valid for as long as it's used (no dangling pointers).
- If you do mut v (vec!), for x in &v: v.push(*x) -> push takes a mutable reference, otherwise there would be a dangling pointer when you resize the array.
- Immutable reference is a copy, mutable refernce is a move. Enforces XOR.
- Footnote: When you pass &mut x to a function, the compiler does an automatic reborrow i.e. shortens the original's lifetime to the call then hands it back.

Next session: Vectors and Matrices for Muon.
  - Option<T> / Result<T, E> and match (replaces null + exceptions)                                    
  - Vec<T> and slices &[T] (the workhorse for numerical code)
  - Traits — for things like Add, Mul, generics over numeric types  