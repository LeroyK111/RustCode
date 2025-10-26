/*
Either枚举有两个变体Left和Right，它具有多种方法和特性，便于使用该枚举进行工作。
*/

use either::Either;

#[test]
fn test() {
    let values = vec![
        Either::Left(1),
        Either::Right(true),
        Either::Left(10),
        Either::Right(false),
    ];
    assert_eq!(
        values
            .into_iter()
            .map(|int_or_bool| -> Either<i32, bool> {
                let int = either::try_left!(int_or_bool);
                Either::Left(int * 2)
            })
            .map(|int_or_bool| { either::for_both!(int_or_bool, s => s.to_string()) })
            .collect::<Vec<_>>(),
        ["2", "true", "20", "false"]
    );
}