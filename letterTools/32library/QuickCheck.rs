// QuickCheck是一个基于属性的测试框架。它允许测试带有大量任意输入数据的代码。如果发现了错误，它会自动找到最小的测试用例来重现错误。
#[cfg(test)]
mod tests {
    fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
        let mut rev = vec!();
        for x in xs {
            rev.insert(0, x.clone())
        }
        rev
    }

    #[quickcheck]
    fn double_reversal_is_identity(xs: Vec<isize>) -> bool {
        xs == reverse(&reverse(&xs))
    }
}