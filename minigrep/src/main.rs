// 调用函数，使用外部传参
use std::env;
// use std::error::Error;
// use std::fs;
use minigrep::Config;
use std::process;

/*
main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导。这些过程有如下步骤：

将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。
当命令行解析逻辑比较小时，可以保留在 main.rs 中。
当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。
经过这些过程之后保留在 main 函数中的责任应该被限制为：

使用参数值调用命令行解析逻辑
设置任何其他的配置
调用 lib.rs 中的 run 函数
如果 run 返回错误，则处理这个错误
这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试他们。仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。让我们遵循这些步骤来重构程序。
*/

fn main() {
    // 需要一个 Rust 标准库提供的函数 std::env::args。这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）
    // let args: Vec<String> = env::args().collect();
    // 打印到异常控制台
    // dbg!(args);
    /*
    [src/main.rs:7] args = [
        "target/debug/minigrep", // * 这是二进制文件名称
        "searchstring", // * 第一个参数
        "example-filename.txt", // * 第二参数
    ]˝
    */

    // 获取参数
    // let pathName = &args[0];
    // let query = &args[1];
    // let file_path = &args[2];
    // println!("zero{}", pathName);
    // println!("one{}", query);
    // println!("two{}", file_path);

    // 读写文件
    // println!("In file {}", file_path);
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("With text:\n{contents}");

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {err}");
        // ! 使用eprintln! 可以让错误信息不被输出
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    if let Err(e) = minigrep::run(config) {
        // println!("Application error: {e}");
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// struct Config {
//     query: String,
//     file_path: String,
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;
//     println!("With text:\n{contents}");
//     Ok(())
// }
