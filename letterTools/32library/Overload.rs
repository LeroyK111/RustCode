// Overload crate提供了一个宏，可以更容易地实现操作符重载。当需要更复杂的操作符实现时，此crate非常有用。在下面的示例中，我们生成了两个不同类型之间的算术计算，其实际工作形式为a * b + 1。
use overload::overload;
use std::ops;

#[derive(PartialEq, Debug)]
struct A {
    v: i32,
}

#[derive(PartialEq, Debug)]
struct B {
    v: i32,
}

overload!((a: ?A) + (b: ?B) -> B { B { v: a.v * b.v + 1 } });

#[test]
fn test() {
    assert_eq!(&A { v: 3 } + B { v: 5 }, B { v: 16 });
}
