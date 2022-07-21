```rust
use std::future::Future;

async fn example<F, Fut>(f: F)
where
    F: FnOnce(i32, i32) -> Fut,
    Fut: Future<Output = bool>,
{
    f(1, 2).await;
}
```

```rust
use std::future::Future;

async fn example<Fut>(f: impl FnOnce(i32, i32) -> Fut)
where
    Fut: Future<Output = bool>,
{
    f(1, 2).await;
}
```

```rust
#![feature(async_closure)]

use futures::future;
use futures::Future;
use tokio;

pub struct Bar;

impl Bar {
    pub fn map<F, T>(&self, f: F)
    where
        F: Fn(i32) -> T,
        T: Future<Output = Result<i32, i32>> + Send + 'static,
    {
        tokio::spawn(f(1));
    }
}

async fn foo(x: i32) -> Result<i32, i32> {
    println!("running foo");
    future::ok::<i32, i32>(x).await
}

#[tokio::main]
async fn main() {
    let bar = Bar;
    let x = 1;

    bar.map(foo);

    bar.map(async move |x| {
        println!("hello from async closure.");
        future::ok::<i32, i32>(x).await
    });
}
```

The traits each represent more and more restrictive properties about closures/functions, indicated by the signatures of their call_... method, and particularly the type of self:
- FnOnce (self) are functions that can be called once
- FnMut (&mut self) are functions that can be called if they have &mut access to their environment
- Fn (&self) are functions that can be called if they only have & access to their environment

A closure |...| ... will automatically implement as many of those as it can.
- All closures implement FnOnce: a closure that can't be called once doesn't deserve the name. Note that if a closure only implements FnOnce, it can be called only once.
- Closures that don't move out of their captures implement FnMut, allowing them to be called more than once (if there is unaliased access to the function object).
- Closures that don't need unique/mutable access to their captures implement Fn, allowing them to be called essentially everywhere.
These restrictions follow directly from the type of self and the "desugaring" of closures into structs; described in my blog post Finding Closure in Rust.

For information on closures, see Closures: Anonymous Functions that Can Capture Their Environment in The Rust Programming Language.
