# Vectors
5/9/2026

- ! is a macro
- fat pointer: pointer + metadata
- Vec<T> is 3 words: pointer, length, capacity.
- &[T] Slice is a fat pointer, always contiguous. Needs pointer, length. This borrows a contiguous range.
- [T]: sequence of type T
- striding can be handled in a ndarray::ArrayView. strides = cache misses, and doesnt allow SIMD/autovectorization i.e. cant load multiple values in a single instruction. And loop is a pointer bump as opposed to multiplies involved if it was strided. so stride only makes sense for large tensors where the cost of copying/reshuffling data is large. 
- segfault = panicked lol. but not really: segfault - unhandled os signal. panic: controlled language level abort.
- Option<T>:
    - first(), pop(), get() can be Null. Rust encodes the return type with Option<T> which can be Some(T) or None, and the compiler forces you to handle the None case before you can use the return value
    - Rust's design slogan: make the implicit explicit, and let the type system police it.
    - * is Mul, + is Add. Traits, not built-ins.
- Ways to use handle Option<T>
    - match can be used to untangle Option<T>. Option is implemented as an Enum, match checks the shape.handle every branch -> or the compiler will yell. match sth { opt1 => do sth, opt 2 => do sth else, ...}
    - .unwrap_or(default_val)
    - if let Some(&n) = v.first() {}  -> if not None
    - .map(|input| operation) -> like map in functional code. if None, None, else, do stuff. map on Option<T> returns Option<T>
- Owned vs borrowed in Option:
    - v.first() -> Option<&T>: borrowed ref to 1st element
    - v.get(i) -> Option<&T>: borrowed ref to ith
    - v.pop() -> Option<T>: owned value, removed from vec.
- Ok, Err, Some, None are called variants.
- Result<T, E>:
    - two shapes: Ok(T) or Err(E). E can be any type like string, enum of error variants, custom struct. No stack unwinding or try catch. Gives the compiler more information on exception handling logic.
    - fn parse(s: &str) -> Result<i32, ParseIntError>. To encode multiple Error modes, you make an enum type and do -> Result<T, ErrorEnum>.
    - Similar principle as option. Make the implicit explicit.
- Handling Result<T, E> looks identical to handling Option. You use match and handle the Ok and Err branches explicitly.
-  The Try operator ?: Chained Result<T, E>: to avoid nested match sth, handle Ok, handle Err, you can use ?. ? visually tells you where failure can pop out and lets you code the happy path cleanly. ? is exactly equivalent to match. On the Err(e) branch, it does a return Err(e.into()).
- into() auto converts e to the target return type expected by enclosing function. To tell how to convert between error types, you implement a From trait. the std lib contains a translation from into() to leverage the From definition.
```
    enum MyError {
        Io(IoError),
        Parse(ParseError),
    }
    impl From<IoError> for MyError {
      fn from(e: IoError) -> Self {
          MyError::Io(e)
      }
    }
    impl From<ParseError> for MyError {
      fn from(e: ParseError) -> Self {
          MyError::Parse(e)
      }
  }
```
- Io, Parse above are variants, you can define any variant. Close to Union[] in python or std::variant. But first class.
- ? is implemented by the Try trait. It preserves the typed contract of what can fail. Forces you to enumerate all types of error modes something can run into.

- Next session: Traits, generics, structs
    - struct: named-field, tuple, unit
    - Define a trait. Implement it. Derive one.
    - Generics with trait bounds: function that works for both f32 and f64.
    - The `*` error revisited: how Mul is implemented, what "trait bound not satisfied" really means.
    - Why Rust separates UB (segfaults) from defined failure (panics) — what does that buy the borrow checker statically?
    - Static dispatch (impl Trait) vs dynamic dispatch (dyn Trait) — what `Box<dyn Error>` is actually doing.
