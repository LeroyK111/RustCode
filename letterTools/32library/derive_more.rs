// NewType模式在Rust中非常常见。有时需要在我们自己的结构中封装第三方库类型：
pub struct NonEmptyVec(Vec<i32>);
// derive_more，本质上是样板代码，因为它复制了Rust内部trait的现有实现，因此不需要我们自己去实现这些trait。我们只需为封装器结构添加派生宏的使用：

#[derive(derive_more::AsRef, derive_more::Deref, derive_more::IntoIterator, derive_more::Index)]
pub struct NonEmptyVec(Vec<i32>);

// 检查这是否有效：
fn collector(iter: impl IntoIterator<Item = i32>) -> Vec<i32> {
    iter.into_iter().collect()
}

#[test]
fn non_emtpy_vec() -> Result<()> {
    assert!(NonEmptyVec::new(vec![]).is_err());

    let non_empty = NonEmptyVec::new(vec![1, 2, 3])?;
    assert_eq!(non_empty.as_ref(), &[1, 2, 3]);
    assert_eq!(non_empty.deref(), &[1, 2, 3]);
    assert_eq!(non_empty[1], 2);
    assert_eq!(collector(non_empty), vec![1, 2, 3]);
    Ok(())
}
// 这个crate包含了用于生成转换特征(From, IntoIterator, AsRef等)，格式化特征(类似display)，操作符特征(Add, Index等)的宏。