/*
Thiserror crate提供了一个宏用于在结构体和枚举上实现std::error::Error特性。从错误处理的角度来看，有两种类型的crate：库和应用程序。库是作为第三方依赖项创建的，将在应用程序中使用。对于库crate，重要的是调用代码可以检查库代码中发生了什么类型的错误，并针对不同类型的错误实现不同的行为。对于应用程序来说，错误的具体类型通常并不重要，因此应用程序函数通常返回Result<T, anyway::Error>类型，因为anyway允许使用?操作符或From trait。Thiserror crate主要用于在库crate中方便地实现错误。

在上面的例子中，std::num::ParseIntError错误被转换为SomeError::ParseInt。如果没有Thiserror这个库，我们将不得不手动编写所有这些转换。
*/

#[derive(thiserror::Error, Debug)]
pub enum SomeError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("int parsing error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("unknown error")]
    General(#[from] anyhow::Error),
}

/// library func
fn int_error(s: &str) -> Result<i32, SomeError> {
    let num = i32::from_str_radix(s, 10)?;
    Ok(num + 2)
}

#[test]
fn test() {
    // application code
    assert!(matches!(int_error("abc").unwrap_err(), SomeError::ParseInt(_)));
    assert!(matches!(
        std::io::Error::new(std::io::ErrorKind::Other, "oh no!").into(),
        SomeError::Io(_)
    ));
}