## Short Explanation
- For maximum flexibility, store callbacks as boxed FnMut objects, and callback setters are generic across callback types.

## Detailed Explanation
### "Function pointers": `callbacks` as fn
- Declare the callback as type fn. fn encapsulates the function defined by the fn keyword, much like a C++ function pointer:
```rust
type Callback = fn();

struct Processor {
    callback: Callback,
}

impl Processor {
    fn set_callback(&mut self, c: Callback) {
        self.callback = c;
    }

    fn process_events(&self) {
        (self.callback)();
    }
}

fn simple_callback() {
    println!("hello world!");
}

fn main() {
    let p = Processor {
        callback: simple_callback,
    };
    p.process_events(); // hello world!
}
```
- This code could be extended to include an Option<Box<Any>> to hold the "user data" associated with the function. Even so, it would not be idiomatic Rust. The Rust way to associate data with a function is to capture it in an anonymous closure, just like in modern C++. Since closures are not fn, set_callback will need to accept other kinds of function objects.
  
### `Callbacks` as generic function objects
- In both Rust and C++ closures with the same call signature come in different sizes to accommodate the different values they might capture. Additionally, each closure definition generates a unique anonymous type for the closure's value. Due to these constraints, the struct cannot name the type of its callback field, nor can it use an alias.
  
- One way to embed a closure in the struct field without referring to a concrete type is by making the struct `generic`. The struct will automatically adapt its size and the type of callback for the concrete function or closure you pass to it:
```rust
struct Processor<CB>
where
    CB: FnMut(),
{
    callback: CB,
}

impl<CB> Processor<CB>
where
    CB: FnMut(),
{
    fn set_callback(&mut self, c: CB) {
        self.callback = c;
    }

    fn process_events(&mut self) {
        (self.callback)();
    }
}

fn main() {
    let s = "world!".to_string();
    let callback = || println!("hello {}", s);
    let mut p = Processor { callback };
    p.process_events();
}
```
- As before, `set_callback()` will accept functions defined with fn, but this one will also accept closures as `|| println!("hello world!")`, as well as closures that capture values, such as `|| println!("{}", somevar)`. Because of this the processor doesn't need userdata to accompany the callback; the closure provided by the caller of `set_callback` will automatically capture the data it needs from its environment and have it available when invoked.
  
- But what's the deal with the FnMut, why not just Fn? Since closures hold captured values, Rust's usual mutation rules must apply when calling the closure. Depending on what the closures do with the values they hold, they are grouped in three families, each marked with a trait:
