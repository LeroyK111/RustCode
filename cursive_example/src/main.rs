use cursive::views::TextView;

fn main() {
    // 创建一个Cursive对象
    let mut siv = cursive::default();

    // 添加一个全局回调，当按下'q'时退出应用程序
    siv.add_global_callback('q', |s| s.quit());

    // 添加一个TextView与我们的消息作为一个新的图层
    siv.add_layer(TextView::new("Hello TUI! 按<q>退出."));

    // 执行Cursive对象
    siv.run(); 
}
