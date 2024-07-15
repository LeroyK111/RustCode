use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 我们将定义每个线程为赢得比赛而必须执行的函数，完成此任务并返回的线程将保存其名称或者id作为赢家。
// 导入必要的库特定工具，这将有助于我们构建游戏。

// thread_task函数将接受3个参数，分别是：

// id：线程Id，这样我们就知道哪个线程正在执行任务。

// max_count：计算每个线程必须完成多少米才能赢得比赛，或者我们可以称之为每个线程必须跑多少米才能赢得比赛。

// pb：更新进度条，这样我们可以直观地看到每个线程的执行情况。

// 在函数内部，我们创建了一个随机延迟，它将随机地持有每个线程几毫秒，将其视为每个线程将具有不同的速度。我们无法控制它，所以我们可以看到一些线程的竞争。

fn run_race(
    winner: Arc<Mutex<Option<usize>>>,
    num_threads: usize,
    max_count: u32,
    mp: Arc<MultiProgress>,
) -> usize {
    (0..num_threads).into_par_iter().for_each(|id| {
        let pb = mp.add(ProgressBar::new(max_count as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "[{{elapsed_precise}}] {{bar:40.cyan/blue}} Thread {}: {{pos}}/{{len}} {{msg}}",
                    id
                ))
                .unwrap()
                .progress_chars("##-"),
        );

        let pb = Arc::new(pb);
        let result = thread_task(id, max_count, pb);

        let mut winner_guard = winner.lock().unwrap();
        if winner_guard.is_none() {
            *winner_guard = Some(result);
        }
    });

    winner.lock().unwrap().unwrap()
}


fn thread_task(id: usize, max_count: u32, pb: Arc<ProgressBar>) -> usize {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(1..=50);

    for _ in 0..max_count {
        pb.inc(1);
        thread::sleep(Duration::from_millis(delay as u64));
    }
    pb.finish_with_message("done");

    id
}

// Arc跟踪我们在循环中对winner变量的引用，Mutex确保只允许单个线程访问通过lock方法向winner变量写入数据。

// 首先，0…Num_threads和into_par_iter将在计数器进程中生成线程。然后，我们将分配每个线程必须完成的任务以赢得比赛。

// 我们将创建一个进度条并将其添加到多进度条中，这将使所有进度条保持在一起并显示每个进度条的进度。

// 首先完成任务的线程将通过锁方法控制赢家变量，因此其他线程无法访问它，直到控制线程完成。然后我们的系统将检查是否有其他线程在上面写了自己的名字。如果是，那么他们不会让其他人重写它。所以只能包含获胜线程的名称。



fn main() {
    let num_threads = 5;
    let max_count = 100;

    println!("Welcome to the Thread Race Game!");
    println!("There are {} threads racing to complete their task.", num_threads);
    print!("Place your bet on which thread will win (0-{}): ", num_threads - 1);
    io::stdout().flush().unwrap();

    let mut bet = String::new();
    io::stdin().read_line(&mut bet).expect("Failed to read line");
    let bet: usize = bet.trim().parse().expect("Please enter a valid number");

    println!("Starting the race...");

    let winner: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
    let mp = Arc::new(MultiProgress::new());

    let winner = run_race(winner, num_threads, max_count, mp);

    println!("Thread {} wins the race!", winner);
    if winner == bet {
        println!("Congratulations! Your bet was correct!");
    } else {
        println!("Sorry, better luck next time.");
    }
}

/*
在这里，我们将设置参与者的数量和比赛的距离。然后显示参与的选手，并要求用户下注其中一个。

将winder变量初始化为None，并将其初始化为race，以便第一个完成比赛的参赛者可以用他们的名字更新它。我们还创建了一个MultiProgress (mp)变量来保存所有参与者的进度，并将其可视化地显示在屏幕上。
*/