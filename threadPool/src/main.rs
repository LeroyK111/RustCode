use std::sync::{Arc, Mutex};
use std::thread;
/*
使用Arc和Mutex来保证线程池线程安全

使用std::thread可以生成新的线程
*/

#[macro_use]
extern crate fstrings;

/*
在这段代码中，WebRequest包含一个字段work，它是一个Box封装的闭包。为什么要使用Box？因为闭包的大小是动态的，换句话说，它的大小在编译时是未知的，所以我们需要将它存储在像Box这样的堆分配容器中。Send和Sync特性向编译器表明，这个特定的闭包可以安全地跨多个线程发送和访问。

构造函数接受闭包作为它的唯一参数。当然，它必须满足与结构体中字段相同的约束。静态生命周期是必需的，因为闭包可能比定义它的作用域活得更长。
*/
struct WebRequest {
    work: Box<dyn FnOnce(&str) + Send + Sync>,
}

impl WebRequest {
    fn new<F>(f: F) -> Self
    where
        F: FnOnce(&str) + Send + Sync + 'static,
    {
        WebRequest { work: Box::new(f) }
    }
}

/*
workers向量，它表示工作线程集合。每个元素都是一个线程的句柄。我们需要持有这个句柄，以便等待线程的完成。
queue，这是一个任务队列，每个任务由一个工作线程执行。Arc允许在多个线
*/

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    queue: Arc<Mutex<Vec<WebRequest>>>,
}

/*
此方法使用指定的线程数初始化池，创建队列。之后，构造函数生成工作线程。这些线程进入一个循环，弹出队列的任务，并执行它们。如果队列恰好为空，则工作线程中断循环。
*/
impl ThreadPool {
    fn new(num_threads: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(num_threads);
        let queue = Arc::new(Mutex::new(Vec::<WebRequest>::new()));

        for i in 0..num_threads {
            let number = f!("Request {i}");
            let queue_clone = Arc::clone(&queue);
            workers.push(thread::spawn(move || loop {
                let task = queue_clone.lock().unwrap().pop();
                if let Some(task) = task {
                    (task.work)(&number);
                } else {
                    break;
                }
            }));
        }
        ThreadPool { workers, queue }
    }

    // execute 这个方法只是用指定的闭包创建一个新的WebRequest，并将其push到任务队列中。
    fn execute<F>(&self, f: F)
    where
        F: FnOnce(&str) + Send + Sync + 'static,
    {
        let task = WebRequest::new(f);
        self.queue.lock().unwrap().push(task);
    }

    // 这是一个阻塞操作，等待线程完成。
    fn join(self) {
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}


/*
正如你所看到的，这种模式非常灵活，但是使用时，请考虑以下影响性能和资源因素：

如果程序销毁线程的速度太慢，或者销毁被阻塞，这可能会使其他线程缺乏资源。


如果创建太多线程，创建未使用的线程会浪费资源和时间。


如果线程创建时间过长，则会影响性能。


销毁太多的线程可能会在以后需要重新创建它们时耗费时间。


总而言之，找到正确的线程数有时可能非常清楚，有时需要使用一些尝试和错误来找到最佳数量。更高级的做法是可以根据需求动态地增加和减少可用线程的数量。
*/

fn main() {
    let pool = ThreadPool::new(6);
    for i in 0..6 {
        pool.execute(move |message| {
            println!("Task: {} prints  {}",i, message);
        });
    }
    pool.join();
}