use std::pin::Pin;

pub struct Foo {
    var: i32,
}

pub struct FooRef<'i> {
    ref_var: &'i i32,
    other: i32,
}

pub struct SimpleRef<'data> {
    foo: Box<Foo>,
    bar: Option<FooRef<'data>>,
}

impl<'data> SimpleRef<'data> {
    fn new() -> SimpleRef<'data> {
        SimpleRef {
            foo: Box::new(Foo { var: 42 }),
            bar: None,
        }
    }

    fn init(mut self: Pin<SimpleRef<'data>>) {
        let this: &mut SimpleRef = unsafe { Pin::get_mut(&mut self) };
        let a = FooRef {
            ref_var: &this.foo.var,
            other: 12,
        };
        this.bar = Some(a);
    }
}

fn main() {}
