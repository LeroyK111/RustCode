
/*
todo: 作为一个三方库进行调用.
*/





pub fn add(left: u64, right: u64) -> u64 {
    println!("adding {} and {}", left, right);

    // 返回值
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

