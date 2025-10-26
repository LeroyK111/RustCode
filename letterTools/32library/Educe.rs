// Educe crate提供了过程性宏，用于更快、更灵活和声明性地实现标准库中的Trait，如Debug、Eq、Ord、Deref等。灵活性在于能够从实现中排除不需要的字段，包括trait边界等。
#[derive(educe::Educe)]
// 注意' new '：生成' new() '方法，调用了Default
#[educe(Default(new))]
#[derive(Debug, PartialEq)]
struct Struct {
    #[educe(Default = 3)]
    f1: u8,
    #[educe(Default = true)]
    f2: bool,
    #[educe(Default = "Hello")]
    f3: String,
}

#[test]
fn test() {
    let expected = Struct {
        f1: 3,
        f2: true,
        f3: String::from("Hello"),
    };
    assert_eq!(Struct::default(), expected);
    assert_eq!(Struct::new(), expected);
}