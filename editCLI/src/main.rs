/*ps
Remove-Item Env:IGNORE_CASE

将标准输出定向到文件中
$Env:IGNORE_CASE=1; cargo run -- to .\assets\poem.txt > .\assets\error.log

*/

/*
-- 之后的值 是 传递给程序的
-- 之前的值 是 传递给 cargo
*/

use std::env;
use std::error::Error;
use std::fs;
use std::process;
// 简易用法
use editCLI::{search, search_case_insensitive};
// mod lib;
// use lib::search;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/// # 这里其实是描述语法


impl Config {
    // fn build(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }

    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     // 判断这个环境变量
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();

    //     Ok(Config {
    //         query,
    //         file_path,
    //         ignore_case,
    //     })
    // }

    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };


        // 判断这个环境变量
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    // 将接收的命令行参数转为 集合
    // let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {config.query}");
    // println!("In file {config.file_path}");

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

