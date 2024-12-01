// Heck crate用于将文本转换为各种常用的变量命名样式，如CamelCase、snake_case等。
use heck::ToShoutyKebabCase;

#[test]
fn test() {
    assert_eq!("i am very angry!".to_shouty_kebab_case(), "I-AM-VERY-ANGRY");
}