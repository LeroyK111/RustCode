/*
1，读取线程的ID

2，单个Select语句中的Case优先级排序

3，具有指针和值接收器的泛型类型
*/

use tokio::sync::mpsc;
use tokio::select;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel::<()>(1);
    drop(tx1);
    let (tx2, mut rx2) = mpsc::channel::<()>(1);
    drop(tx2);

    let mut n_ready1 = 0;
    let mut n_ready2 = 0;
    let n = 10_000;
    for _ in 0..n {
        select! {
            biased; // 按出现的顺序优先处理已准备好的case
            _ = rx1.recv() => {
                n_ready1 += 1;
            },
            _ = rx2.recv() => {
                n_ready2 += 1;
            },
        }
    }

    println!("n_ready1: {}", n_ready1);
    println!("n_ready2: {}", n_ready2);
}

trait MyTrait {
    fn M(&self);
    fn P(&mut self);
}

struct S;

impl MyTrait for S {
     fn M(&self) {}
    fn P(&mut self) {}
}

fn f<T: MyTrait>(t: &mut T) {
    t.M();
    t.P();
}
