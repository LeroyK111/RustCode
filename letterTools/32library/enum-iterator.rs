// 用于在枚举或结构体的值上生成迭代器的宏。
use enum_iterator::{all, first, last, next, Sequence};
use itertools::Itertools;

#[derive(Debug, PartialEq, Sequence)]
enum Direction {
    Left,
    Middle,
    Right,
}

#[test]
fn test_enum() {
    use Direction::*;

    assert_eq!(all::<Direction>().collect_vec(), vec![Left, Middle, Right]);
    assert_eq!(first::<Direction>(), Some(Left));
    assert_eq!(last::<Direction>(), Some(Right));
    assert_eq!(next(&Middle), Some(Right));
}

#[derive(Debug, PartialEq, Sequence)]
struct Foo {
    a: bool,
    b: u8,
}

#[test]
fn test_struct() {
    let expected_number_of_elements = 512;
    assert_eq!(
        enum_iterator::cardinality::<Foo>(),
        expected_number_of_elements
    );
    assert_eq!(first::<Foo>(), Some(Foo { a: false, b: 0 }));
    assert_eq!(last::<Foo>(), Some(Foo { a: true, b: 255 }));
}