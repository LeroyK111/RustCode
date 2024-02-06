use std::{thread, time};
// 测试过程宏
use function_benchmark_macro::auto_log;

#[auto_log]
fn log_function() {
    for num in 0..10 {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", num);
    }
}

fn main() {
    log_function();
}
