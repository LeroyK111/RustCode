/*
线程池是如何工作的？在线程池体系结构中，主线程只有两个任务：1，接收所有的任务并将它们存储在一个地方。2，创建多个线程，并定期为它们分配不同的任务。在接收任务之前创建线程集，并使用ID存储在某个地方，以便我们可以通过ID识别它们。然后每个线程都在等待接收任务，如果它们得到任务，就开始处理任务。完成任务后，他们再次等待下一个任务。当该线程忙于执行任务时，主线程将更多的任务分配给其他线程，这样在主线程结束任务之前没有线程空闲。在完成所有任务后，主线程终止所有线程并关闭线程池。现在我们了解了线程池是如何工作的。接下来，让我们使用Rust实现一个线程池。
*/

struct Worker {
    /*
    我们需要一个函数来生成一个线程并返回它的JoinHandle。此外，我们需要知道线程的ID，如果我们搞砸了，就可以用线程ID记录错误，这样我们就可以知道哪个线程出错了。可以看出，如果两个相互关联的数据需要组合，需要一个结构体。我们来创建一个：
    */
    id: usize,
    thread: JoinHandle<()>
}

impl Worker {
    /*
    现在我们实现一个可以返回新Worker的构造函数：
    现在，我们的函数已经准备好创建线程并将它们返回给调用者。
    */
    fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});

        Self {id, thread}
    }
}

/*
2. 存放线程我们需要一个结构来保存所有线程的所有JoinHandles，我们还想控制线程池可以拥有多少线程。这意味着，我们需要一个带有构造函数的结构体，该函数指定一个数字来指示线程的数量，并且必须调用Worker来创建线程。
*/

struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0, "Need at least 1 worker!");

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i));
        }

        Self { workers }
    }
}

// 我们有了创建线程和管理线程的函数，现在是时候创建一个可以将任务分配给不同线程的函数了。


/*
3. 给线程分配任务我们的线程池结构体必须有一个函数，该函数可以在线程内部分配和执行任务。但是有一个问题，我们如何将任务发送给线程，以便线程能够执行任务？为此，我们需要一个task类型来表示我们需要完成的任务：
*/

type task = Box<dyn FnOnce() + Send + 'static>;

/*
在这里，意味着我们的任务必须实现Box<dyn>里的这些Trait：
1，实现FnOnce()意味着我们的任务是一个只能运行一次的函数。
2，实现Send，因为我们将任务从主线程发送到工作线程，所以将任务设置为Send类型，以便它可以在线程之间安全地传输。
3，实现'static，意味着我们的任务必须和程序运行的时间一样长。
现在是时候将任务发送给工作线程了，但要做到这一点，我们必须在主线程和所有工作线程之间建立一个通道，因此我们需要使用Arc<Mutex<()>>。让我们来更新这两个构造函数：
*/

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0, "Need at least 1 worker!");

        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&reciever)));
        }

        Self {
            workers,
            sender: Some(sender)
        }
    }
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<Receiver<Task>>>) -> Self {
        let thread = thread::spawn(move || {});

        Self {
            id,
            thread
        }
    }
}

/*
在ThreadPool构造函数中，我们创建了一个新的通道，并在Arc<Mutex<()>>中封装了接收器，我们把接收器发送给工作线程，以便主线程可以发送任务，工作线程可以接收任务。此外，我们必须更新ThreadPool结构体，以包含一个发送者，它将被主线程用来向不同的线程发送任务。现在，让我们实现在工作线程中执行任务的逻辑：
*/

fn new(id: usize, reciever: Arc<Mutex<Receiver<task>>>) -> Self {
    let thread = thread::spawn(move || {
        loop {
            let receiver = reciever.lock()
                .expect("Failed to grab the lock!")
                .recv();

            match receiver {
                Ok(task) => {
                    println!("Thread {} got the task& executing.", id);
                    task();
                    thread::sleep(Duration::from_millis(10));
                },

                Err(_) => {
                    println!("No got the task");
                    break;
                }
            }
        }
    });

    Self {
        id,
        thread
    }
}

// 这里，在每个循环中，我们都试图获得锁并调用锁上的recv()，以便我们可以获得主线程发送的任务。接下来，我们在ThreadPool中实现一个函数，将任务发送到不同的线程。

impl ThreadPool {
    fn new(size: usize) -> Self {
        // snip
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(job);

        self.sender.send(job)
            .expect("Failed to send the job to workers!");
    }
}

// 我们还需要创建一个函数，在ThreadPool结束时动态终止所有线程。简单地说，我们必须手动实现ThreadPool的Drop特性，在那里我们将终止所有线程。

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Thread {} is shutting down.", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join()..unwrap_or_else(|_| panic!("Failed to join the thread {}", worker.id));}
        }
    }
}

// 这里我们还必须删除发送方，因为如果我们不这样做，那么接收方将永远循环。如果删除发送者，那么接收者也会自动删除，我们就可以成功地退出这个程序。


fn main() {
    let pool = ThreadPool::new(5);

    for _ in 0..10 {
        pool.execute(|| println!("Doing something"));
    }
}