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
  
    - `Fn` are closures that only read data, and may be safely called multiple times, possibly from multiple threads. Both above closures are Fn.
    
    - `FnMut` are closures that modify data, e.g. by writing to a captured mut variable. They may also be called multiple times, but not in parallel. (Calling a FnMut closure from multiple threads would lead to a data race, so it can only be done with the protection of a mutex.) The closure object must be declared mutable by the caller.
    
    - `FnOnce` are closures that consume some of the data they capture, e.g. by passing a captured value to a function that takes it by value. As the name implies, these may be called only once, and the caller must own them.
    
- Somewhat counter-intuitively, when specifying a trait bound for the type of an object that accepts a closure, `FnOnce` is actually the most permissive one. Declaring that a generic callback type must satisfy the FnOnce trait means that it will accept literally any closure. But that comes with a price: it means the holder is only allowed to call it once. Since process_events() may opt to invoke the callback multiple times, and as the method itself may be called more than once, the next most permissive bound is `FnMut`. Note that we had to mark process_events as mutating self.

### Non-generic callbacks: function trait objects
  - Even though the generic implementation of the callback is extremely efficient, it has serious interface limitations. It requires each Processor instance to be parameterized with a concrete callback type, which means that a single Processor can only deal with a single callback type. Given that each closure has a distinct type, the generic Processor cannot handle `proc.set_callback(|| println!("hello"))` followed by `proc.set_callback(|| println!("world"))`. Extending the struct to support two callbacks fields would require the whole struct to be parameterized to two types, which would quickly become unwieldy as the number of callbacks grows. Adding more type parameters wouldn't work if the number of callbacks needed to be dynamic, e.g. to implement an `add_callback` function that maintains a vector of different callbacks.

  - To remove the type parameter, we can take advantage of trait objects, the feature of Rust that allows automatic creation of dynamic interfaces based on traits.
  
  - A trait object is created by borrowing an object with the & operator and casting or coercing it to a reference to the specific trait. In this case, since Processor needs to own the callback object, we cannot use borrowing, but must store the callback in a heap-allocated `Box<dyn Trait>`, which is functionally equivalent to a trait object.
    
  - If Processor stores `Box<dyn FnMut()>`, it no longer needs to be generic, but the `set_callback` method now accepts a generic `c` via an impl Trait argument. As such, it can accept any kind of callable, including closures with state, and properly box it before storing it in the Processor. The generic argument to `set_callback` doesn't limit what kind of callback the processor accepts, as the type of the accepted callback is decoupled from the type stored in the Processor struct.
```rust
struct Processor {
    callback: Box<dyn FnMut()>,
}

impl Processor {
    fn set_callback(&mut self, c: impl FnMut() + 'static) {
        self.callback = Box::new(c);
    }

    fn process_events(&mut self) {
        (self.callback)();
    }
}

fn simple_callback() {
    println!("hello");
}

fn main() {
    let mut p = Processor {
        callback: Box::new(simple_callback),
    };
    p.process_events();
    let s = "world!".to_string();
    let callback2 = move || println!("hello {}", s);
    p.set_callback(callback2);
    p.process_events();
}
```
    
### Lifetime of references inside boxed closures
- The 'static lifetime bound on the type of the `c` argument accepted by `set_callback` is a simple way to convince the compiler that references contained in `c`, which might be a closure that refers to its environment, only refer to global values and will therefore remain valid throughout the use of the callback. But the static bound is also very heavy-handed: while it accepts closures that own objects just fine (which we've ensured above by making the closure move), **it rejects closures that refer to local environment**, even when they only refer to values that outlive the processor and would in fact be safe.
    
- As we only need the callbacks alive as long as the processor is alive, we should try to tie their lifetime to that of the processor, which is a less strict bound than 'static. But if we just remove the 'static lifetime bound from `set_callback`, it no longer compiles. This is because `set_callback` creates a new box and assigns it to the callback field defined as `Box<dyn FnMut()>`. Since the definition doesn't specify a lifetime for the boxed trait object, 'static is implied, and the assignment would effectively widen the lifetime (from an unnamed arbitrary lifetime of the callback to 'static), which is disallowed. The fix is to provide an explicit lifetime for the processor and tie that lifetime to both the references in the box and the references in the callback received by `set_callback`:

```rust
struct Processor<'a> {
    callback: Box<dyn FnMut() + 'a>,
}

impl<'a> Processor<'a> {
    fn set_callback(&mut self, c: impl FnMut() + 'a) {
        self.callback = Box::new(c);
    }
    // ...
}
```
    
- With these lifetimes being made explicit, it is no longer necessary to use 'static. The closure can now refer to the local s object, i.e. no longer has to be moved, provided that the definition of s is placed before the definition of p to ensure that the string outlives the processor.
    
