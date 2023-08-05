use std::fs::File;
use std::io::{self, Read};
use std::net::IpAddr;
pub fn demo() {
    // read_username_from_file();
    // read_username_from_file2();
    // read_username_from_file3();
    // read_username_from_file4();

    // todo
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

// 传播 propagating
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    // 打开文件
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    // 读取文件内容
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 简写错误传播，舒服
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 获取文本内容的第一行最后一个数字
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 再简写
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
// use std::fs;
// use std::io;
// 再次简写
// fn read_username_from_file4() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt");
// }
