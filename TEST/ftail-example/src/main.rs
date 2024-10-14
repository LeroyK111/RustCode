

/*

默认实现ftail crate提供了几个默认的日志记录器，因此你可以根据需要快速定制日志。
输出到终端基本的console日志记录器使用标准格式打印日志消息。它简单、有效，非常适合在终端中进行开发或调试。
输出格式化的日志到终端formatted_console日志记录器为日志添加了一些修饰，包括颜色格式和更清晰的时间戳。这使得日志更容易阅读，特别是在开发期间。有些终端甚至生成了指向日志消息的源文件和行号的可点击链接。



*/
use ftail::Ftail;


use log::LevelFilter;

fn main() {
    Ftail::new()
        .console(LevelFilter::Debug)
        // 记得先创建目录
        .daily_file("E:/RustCode/TEST/ftail-example/logs", LevelFilter::Error)
        .init()
        .unwrap();

    //可以在代码中的任何地方记录消息
    log::trace!("Thisisatracemessage");
    log::debug!("Thisisadebugmessage");
    log::info!(target:"foo","bar");
    log::warn!("Thisisawarningmessage");
    log::error!("Thisisanerrormessage");
}
