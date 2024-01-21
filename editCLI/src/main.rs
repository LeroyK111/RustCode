mod db;
mod utils;
mod handler;

use clap::Parser;
use dotenv::dotenv;
use human_panic::setup_panic;

// Args结构体用于解析命令行参数
#[derive(Parser, Debug)]
#[command(name="MiniKey", author="", version, about="一个简单的键值存储", long_about = None)]
struct Args {
    #[arg(required=false)]
    cmd: Option<String>,

    #[arg(short, long)]
    custom: Option<String>,

    #[arg(short, long)]
    docs: bool,
}

#[tokio::main]
async fn main() {
    // 加载.env文件
    dotenv().ok();

    //human-panic 初始化
    setup_panic!();

    // 打印 logo 
    let logo = r#"
    ___  ____       _   _              
    |  \/  (_)     (_) | |             
    | .  . |_ _ __  _  | | _____ _   _ 
    | |\/| | | '_ \| | | |/ / _ \ | | |
    | |  | | | | | | | |   <  __/ |_| |
    \_|  |_/_|_| |_|_| |_|\_\___|\__, |
                                __/ |
                                |___/ 
    "#;
    bunt::println!("{$green}{}{/$}", logo);

    let args = Args::parse();

    let cmd: String;
    if args.cmd.is_some(){
        cmd = args.cmd.unwrap();
    } else{
        cmd = inquire::Text::new("输入命令: ").with_help_message("输入一个有效命令").with_autocomplete(
            &utils::suggester,
        ).prompt().unwrap();
    }

    // println!("用户输入的命令是：{}", cmd);


    match cmd.as_str() {
        "set" => handler::add().await,
        "list" => handler::list().await,
        "delete" => handler::delete().await,
        "get" => handler::get().await,
        "search" => handler::search().await,
        "exit" => {
            bunt::println!("{$red}Exiting...{/$}");
            std::process::exit(0);
        }
        "help" => todo!("帮助命令还没实现"),
        _ => todo!("命令未找到")
    }
}


