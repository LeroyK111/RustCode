/*
path: errors/useResult.rs
可恢复错误

场景	推荐方式	原因
文件不存在、没有权限、读取失败	Result，用 ? 向上传递	环境问题可能正常发生
用户输入格式错误、数字超出范围	Result，由调用者提示重新输入	输入错误不等于程序有 bug
请求失败、服务限流	Result	调用者可能重试或降级

!更多是使用result

内部关键约束被破坏，无法合理继续	panic! 或断言	更像程序 bug，需要修复
硬编码数据已确定有效，但 API 仍返回 Result	expect	写明为什么确信它会成功
学习示例、临时原型、测试	可以用 unwrap / expect	简化代码，测试中失败就应暴露

*/

use io::ErrorKind;
use std::fs::{self, File};
use std::io;

pub fn controlFile() {
    let greeting_file_result = File::open("./hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("文件不存在时的, 异常处理");
                return;
            }
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // 简写 unwrap_or_else, 只管失败处理
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // 直接使用也行
    // let greeting_file = File::open("hello.txt").unwrap();

    // 自定义报错
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
}

pub fn propagating() {
    /*
      传统写法, 传播错误
    */
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    /*
      ? 快捷方式: 非常常用
    */

    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    fn read_username_from_file4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    /*
             todo 总结
        写法	成功时	失败时	意图
        match	自己写处理逻辑	自己写处理逻辑	完整控制分支

        .unwrap()	取出值	panic!	失败就中断

        .expect("说明")	取出值	带自定义说明的 panic!	失败就中断，并说明预期

        .unwrap_or_else(\|e\| { ... })	取出值	执行闭包	定制失败后的替代处理

        ?	取出值	提前返回错误	交给调用者决定


        main 也可以返回

        fn main() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;

        Ok(())
    }

            */
}
