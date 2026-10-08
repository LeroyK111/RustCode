/*
安装工具,实现热重启

cargo install --locked watchexec-cli
cd E:\RustCode\learning\webServer
watchexec --restart --watch src --watch Cargo.toml -- cargo run
*/

/*
验证 Ctrl+C 优雅停机：

cargo build
.\target\debug\webServer.exe

测试时先直接运行程序，不通过 watchexec 启动。
*/

use webServer::ThreadPool;

use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

// fn main() {
//     // 建立tcp监听端口
//     let listener =
//         TcpListener::bind("127.0.0.1:7878").expect("无法监听 127.0.0.1:7878，请检查端口是否被占用");
//
//     println!("服务器已启动：http://127.0.0.1:7878");
//
//     // 每轮等待一个接入: 这里本质就是轮询处理请求, 单线程
//     for connection in listener.incoming() {
//         match connection {
//             Ok(stream) => match handle(stream) {
//                 Ok(()) => {}
//                 Err(error) => {
//                     eprintln!("处理连接失败：{error}");
//                 }
//             },
//             Err(error) => {
//                 eprintln!("接受连接失败：{error}");
//             }
//         }
//     }
// }

fn main() {
    // 主线程和 Ctrl+C 处理线程共享停止标志。
    let stopping = Arc::new(AtomicBool::new(false));
    let signal = Arc::clone(&stopping);

    // 收到 Ctrl+C 后，只设置停止标志。
    // 不在这里直接结束进程，也不在这里等待工作线程。
    ctrlc::set_handler(move || {
        signal.store(true, Ordering::SeqCst);
    })
    .expect("无法注册 Ctrl+C 处理器");

    let listener =
        TcpListener::bind("127.0.0.1:7878").expect("无法监听 127.0.0.1:7878，请检查端口是否被占用");

    // 使用非阻塞 accept，避免主线程一直等待新连接，
    // 无法重新检查 Ctrl+C 停止标志。
    listener
        .set_nonblocking(true)
        .expect("无法设置监听器为非阻塞模式");

    let pool = ThreadPool::new(4);

    println!("服务器已启动：http://127.0.0.1:7878");
    println!("按 Ctrl+C 停止接受新连接，并等待已有任务完成");

    // 不再使用 incoming().take(2)。
    // 持续接收连接，直到收到停止通知。
    while !stopping.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, address)) => {
                // 刚接受连接时，再检查一次停止标志。
                if stopping.load(Ordering::SeqCst) {
                    drop(stream);
                    break;
                }

                println!("接受连接：{address}");

                // 主线程提交任务，Worker 执行 handle。
                pool.execute(move || match handle(stream) {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("处理连接失败：{error}");
                    }
                });
            }
            Err(error) => {
                match error.kind() {
                    io::ErrorKind::WouldBlock => {
                        // 暂时没有新连接，不是服务器故障。
                        // 稍等后重新检查停止标志。
                        thread::sleep(Duration::from_millis(100));
                    }
                    io::ErrorKind::Interrupted => {
                        // 接受操作被中断，重新检查并尝试。
                        continue;
                    }
                    _ => {
                        eprintln!("接受连接失败：{error}");
                        break;
                    }
                }
            }
        }
    }

    println!("停止接受新连接");

    // 先关闭监听入口。
    drop(listener);

    println!("等待已有任务处理完成");

    // 关闭任务发送端，并等待所有工作线程退出。
    drop(pool);

    println!("服务器已停止");
}

// 流式处理
fn handle(mut stream: TcpStream) -> io::Result<()> {
    // 监听器使用非阻塞模式，
    // 具体连接在 Worker 中仍使用阻塞 I/O。
    stream.set_nonblocking(false)?;

    // 为连接设置读写超时，避免完全不传输数据的客户端
    // 一直占住工作线程。这不是整个请求的总执行期限。
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;

    // 用单独的作用域完成读取。
    let request = {
        // 转入缓冲区
        let reader = BufReader::new(&stream);

        // 获取所有行
        let mut lines = reader.lines();

        // 先读取请求行。
        let request = match lines.next() {
            Some(line) => line?,
            None => {
                // 客户端关闭连接，没有发送请求。
                return Ok(());
            }
        };

        // 继续读取请求头，直到空行。
        for line in lines {
            let line = line?;

            if line.is_empty() {
                break;
            }
        }

        request
    };

    println!("收到请求：{request}");

    // 选择状态行和页面文件。
    // let (status, filename) = match request.as_str() {
    //     // 根据请求状态 返回不同的路由内容
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
    //     _ => ("HTTP/1.1 404 Not Found", "404.html"),
    // };

    // 选择状态行和页面文件。
    let (status, filename) = match request.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            // 这里就导致阻塞了
            println!("慢请求开始，等待 5 秒");

            thread::sleep(Duration::from_secs(5));

            println!("慢请求结束");

            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 Not Found", "404.html"),
    };

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(filename);

    println!("静态文件路径: {}", path.display());

    let contents = fs::read_to_string(&path)?;
    let length = contents.len();

    // 构建响应体: 字符串
    let response = format!(
        "{status}\r\n\
         Content-Length: {length}\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Connection: close\r\n\
         \r\n\
         {contents}"
    );

    // 字符串转为字节码 它会反复尝试写入，
    // 直到整个输入缓冲区写完，或者遇到需要返回的错误；
    // 不像一次 write() 那样可能只写入一部分。
    // ?是确保失败后 可以返回到err
    stream.write_all(response.as_bytes())?;

    Ok(())
}
s