// Derivative crate提供了用于实现标准库中Trait的宏，类似于educe。
#[derive(Derivative)]
#[derivative(PartialEq)]
struct Foo {
    foo: u8,
    #[derivative(PartialEq="ignore")]
    bar: u8,
}

assert!(Foo { foo: 0, bar: 42 } == Foo { foo: 0, bar: 7});
assert!(Foo { foo: 42, bar: 0 } != Foo { foo: 7, bar: 0});