/*
Rust中最流行的模式之一是构建器模式，当需要创建包含许多字段的复杂结构时，此模式非常方便。使用derive_more crate，可以自动生成构建器模式，使代码更简洁：

derive_more crate 还支持构建方法中的字段验证。
*/

#[derive(Debug, Eq, PartialEq, Default, derive_builder::Builder)]
#[builder(pattern = "immutable")]
#[builder(default)]
#[builder(setter(strip_option))]
struct Calculation {
    a: Option<i32>,
    b: Option<i32>,
    c: Option<i32>,
    d: Option<i32>,
    // ... can be more optional fields
}

fn derive_builder() -> Result<()> {
    let builder = CalculationBuilder::default();
    builder.a(1).build()?;
    builder.a(6).d(7).build()?;
    builder.b(2).c(3).build()?;

    Ok(())
}