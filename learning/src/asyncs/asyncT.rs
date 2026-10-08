// use std::pin::Pin;
// use std::task::{Context, Poll};

// 这里就是 futurn 的原型定义 
// pub trait Future {
    /*
        | 部分                   | 含义                          |
    | -------------------- | --------------------------- |
    | type Output          | Future 完成后产生什么类型的值          |
    | poll(...)            | 推进计算，并检查是否已经完成              |
    | self: Pin<&mut Self> | 用一种受约束的可变引用访问 Future，下一步详细讲 |
    | cx: &mut Context<'_> | 提供当前任务的上下文，里面可以取得唤醒器        |
    | Poll<Self::Output>   | 返回“未完成”或“已完成及其输出”           |
        */
//     type Output;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }

/*

async fn calculate() -> u32 {
    42
}

Future<Output = u32>

Poll::Ready(42)

!poll 的本质
pub enum Poll<T> {
    Ready(T),
    Pending,
}

rx.recv().await

运行时推进任务
    ↓
任务等待的 Future 返回 Pending
    ↓
留下“怎样唤醒这个任务”的信息
    ↓
当前任务暂停，执行线程可以做其他工作
    ↓
数据到达或计时器触发，调用唤醒器
    ↓
运行时之后再次推进任务

*/





use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
struct DemoFuture {
    polled_once: bool,
}

impl Future for DemoFuture {
    type Output = u32;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        // 取出可变 属性
        let this = self.get_mut();
        println!("{this:?}");
        if this.polled_once {
            println!("第二次 poll：返回 Ready(42)");

            Poll::Ready(42)
        } else {
            println!("第一次 poll：返回 Pending");

            // 这里我们修改了属性
            this.polled_once = true;

            // 本例没有外部事件来源，所以主动请求再次调度
            // ?请给这个任务一次再次被检查的机会。 只有出去后 才会被受理这个请求
            cx.waker().wake_by_ref();

            // 返回pending 准备被再次调用
            Poll::Pending
        }
    }
}

pub fn test() {
    let future = DemoFuture {
        polled_once: false,
    };

    trpl::block_on(async {
        let result = future.await;

        println!("await 得到结果：{}", result);
    });
}


async fn example() {
    /*
    如果这样的状态机在建立内部引用后被搬到别的位置，引用可能仍指向旧位置。

    因此，需要一种机制保证：已经固定的、依赖地址稳定的 Future，其内部值不会被安全代码移走。 这就是 Pin 要提供的保证。
    */
    let text = String::from("hello");
    let reference = &text;

    trpl::yield_now().await;
    println!("{}", text);
    println!("{}", reference);
}

pub fn test2 () {
    trpl::block_on(async {

        example().await

    })
}



use std::time::Duration;

pub fn test3() {
    trpl::block_on(async {
        let future1 = async {
            println!("任务 A 开始");

            trpl::sleep(Duration::from_millis(500)).await;

            println!("任务 A 完成");
        };

        let future2 = async {
            println!("任务 B 开始");

            trpl::sleep(Duration::from_millis(800)).await;

            println!("任务 B 完成");
        };

        // dyn Future：统一不同 Future 的具体类型
        // Box::pin：在堆上存放并固定 Future
        // 
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![
                Box::pin(future1),
                Box::pin(future2),
            ];

        trpl::join_all(futures).await;
        
        // 
        // trpl::join!(future1, futures);

        println!("所有任务完成");
    });
}






struct Data {
    values: [u8; 16],
}

pub fn moveEle() {
    // !这就是为什么 参数传递过程中 参数的内存位置会发生移动 
    let a = Box::new(Data {
        values: [7; 16],
    });

    println!("a 指向的数据地址：{:p}", &*a);

    // 移动 Box 句柄，没有搬动它指向的数据
    let b = a;

    println!("b 指向的数据地址：{:p}", &*b);

    // 从 Box 中取出 Data 本体
    let inner = *b;

    println!("取出后的数据地址：{:p}", &inner);
    println!("第一个元素：{}", inner.values[0]);
}


// 只有满足 Unpin 的类型才能传进来
fn check_unpin<T: Unpin>(_: &T) {}

pub fn test4() {
    let future = async {
        println!("异步工作");
    };

    // 取消注释会报错：
    // async 块生成的 Future 不满足 Unpin
    // check_unpin(&future);


    // !异步状态机可能依赖自身的内存位置保持稳定，因此 Rust 需要表达“这个值固定后不能随便搬走”。Pin 提供这种约束，Unpin 表示某种类型不需要它
    // 异步状态机, 所以pin的存在 就是为了 保证对象的引用 还是固定的
    let pinned_future = Box::pin(future);

    // 可以编译：
    // 这个 Pin<Box<...>> 指针句柄本身满足 Unpin
    check_unpin(&pinned_future);
}





use std::pin::{pin};


pub fn test5() {
    trpl::block_on(async {
        let future1 = pin!(async {
            println!("A 开始");

            trpl::sleep(Duration::from_millis(500)).await;

            println!("A 完成");
        });

        let future2 = pin!(async {
            println!("B 开始");

            trpl::sleep(Duration::from_millis(800)).await;

            println!("B 完成");
        });

        let future3 = pin!(async {
            println!("C 执行完成");
        });

        //  Box::pin 版本一样：统一不同 Future 的类型，放进集合，再一起等待完成。区别是固定方式和指针类型
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![future1, future2, future3];

        trpl::join_all(futures).await;

        println!("所有 Future 都完成");
    });
}




use trpl::{Stream, StreamExt};

struct NumberStream {
    next: u32,
    ready: bool,
}

impl Stream for NumberStream {
    type Item = u32;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        // 已经输出 1、2、3，结束这个流
        if this.next > 3 {
            println!("poll_next：Ready(None)");
            return Poll::Ready(None);
        }

        // 每个数字都先模拟一次“尚未就绪”
        if !this.ready {
            this.ready = true;

            println!("poll_next：Pending");

            // 本例没有外部事件，所以主动请求再次调度
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        // 本次可以交付一个数字
        let value = this.next;

        this.next += 1;
        this.ready = false;

        println!("poll_next：Ready(Some({}))", value);

        Poll::Ready(Some(value))
    }
}

pub fn test6() {
    trpl::block_on(async {
        let mut stream = NumberStream {
            next: 1,
            ready: false,
        };

        while let Some(value) = stream.next().await {
            println!("使用者收到：{}", value);
        }

        println!("读取结束");
    });
}



use std::{thread};

pub fn test7() {
    /*
    ! 这就是组合使用
    */
    let (tx, mut rx) = trpl::channel();

    let worker = thread::spawn(move || {
        for number in 1..=10 {
            tx.send(number).unwrap();

            thread::sleep(Duration::from_secs(1));
        }

        // 线程闭包结束，tx 被销毁
    });


    
    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("收到：{}", message);
        }

        println!("信道已关闭，接收结束");
    });

    worker.join().unwrap();

    println!("程序结束");
}