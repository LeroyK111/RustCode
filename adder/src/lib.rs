/*
!带入测试。
*/

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    // a + 3
    a + 2
}
// todo 自定义失败信息
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::format("11")
}

// 使用should_panic 检查 panic
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 捕捉异常的枚举结构体，
    #[test]
    fn it_works_enmus() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // 这里则是一个失败的测试例子
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    // 结构体测试
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // 断言 真假 这里报错
        assert!(larger.can_hold(&smaller));
    }

    // 再次测试
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_adds_two() {
        // 断言相等
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // 直接打印自定义错误信息
        assert!(result.contains("Carol"), "我是错误信息{}", result);
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "你不能传入正常值")]
    fn greater_than_100() {
        // 检测panic是否生效
        Guess::new(12);
    }
}
