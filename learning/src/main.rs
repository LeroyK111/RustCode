use ferris_says::say;
use std::io::{stdout, BufWriter};
// 引入包，并且导入say函数

fn test_says() {
    let stdout: std::io::Stdout = stdout();
    let message: &str = "Hello fellow Rustaceans!";
    let width: usize = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(message, width, &mut writer).unwrap();
}

fn main() {
    println!("Hello, Rust!");
    test_says()
}
