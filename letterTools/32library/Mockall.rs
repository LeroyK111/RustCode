/*
Mockall crate为(几乎所有)Trait和结构体提供了自动生成的模拟对象，这些对象可以在单元测试中使用，而不是使用原始类型的对象，这可以使编写高级单元测试或测试复杂的边缘情况变得更容易。
*/

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
trait CalcTrait {
    fn foo(&self, x: u32) -> u32;
}

fn calculation(calc: impl CalcTrait, x: u32) -> u32 {
    calc.foo(x)
}

#[test]
fn test() {
    let mut mock = MockCalcTrait::new();
    mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);

    assert_eq!(5, calculation(mock, 4));
}
/*
可以使用#[automock]属性宏自动生成模拟对象。但是，它有其局限性，因此有时必须使用过程宏mock!手动实现模拟对象。
*/

