use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

// 统一表示“稍后交给工作线程执行的任务”。
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,

    // 使用 Option，允许停机时提前取出并销毁发送端。
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// # Panics
    ///
    /// size 为 0 时会 panic。
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "线程池至少需要一个工作线程");

        // 信道充当任务队列。
        let (sender, receiver) = mpsc::channel::<Job>();

        // 多个 Worker 共享同一个接收端。
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));

            workers.push(worker);
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(task);

        let sender = self.sender.as_ref().expect("线程池已关闭任务发送端");

        sender.send(job).expect("任务发送失败：线程池接收端已关闭");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("线程池开始关闭");

        // 先关闭发送端，告诉 Worker 不会再有新任务。
        drop(self.sender.take());

        // 移出所有 Worker，取得各个 JoinHandle 的所有权。
        for worker in self.workers.drain(..) {
            println!("等待 Worker {} 退出", worker.id);

            match worker.thread.join() {
                Ok(()) => {
                    println!("Worker {} 已回收", worker.id);
                }
                Err(_) => {
                    // 不在 Drop 中因 join 失败再次 unwrap/panic。
                    eprintln!("Worker {} 已异常退出，继续回收其他线程", worker.id);
                }
            }
        }

        println!("线程池关闭完成");
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                // 锁只覆盖“接收任务”阶段。
                let message = {
                    let guard = receiver.lock().expect("任务队列的锁已被污染");

                    guard.recv()
                };

                // 到这里，guard 已被丢弃，锁已释放。
                match message {
                    Ok(job) => {
                        println!("Worker {id} 开始执行任务");

                        job();

                        println!("Worker {id} 完成任务");
                    }
                    Err(_) => {
                        println!("Worker {id}：任务队列已结束，退出");

                        break;
                    }
                }
            }
        });

        Self { id, thread }
    }
}
