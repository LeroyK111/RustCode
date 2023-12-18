/*
! Rust有很多测试策略，从单元测试到集成测试。在本文中，我们将探索使用Insta进行快照测试，了解它如何补充你的开发工作。

!什么是快照测试？

快照测试是一种通过将输出与一组已定义的预期值进行比较来验证代码正确性的方法。例如，如果你以前编写过集成测试，那么可以将部分测试视为快照，因为你正在将预期结果与实际输出进行比较。

默认情况下，Rust使用assert_eq!函数，但它只允许你与原始Rust类型进行比较。快照测试要求对更复杂的数据结构进行比较。

通常，快照测试是在前端而不是后端完成的，因为前端应用程序返回HTML而不是常规字符串。比较输出更省时，而不是解析HTML并检查每个特定元素。

在测试整个程序的输出时，你可以充分利用快照测试，从而测试网页中的更多元素，而不必担心所有结果是否一致。

!Insta仅通过Serde支持CSV、JSON、TOML、YAML和RON文件，Serde是一个数据序列化库，可以将各种类型的数据结构编码为更紧凑的格式，反之亦然。
*/

use std::env;
use std::io::{self, BufRead};
use std::path::Path;

struct Task {
    name: String,
    is_completed: bool,
}

fn readline() -> String {
    let mut strr: String = "".to_string();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        strr = line.unwrap().to_string();
        break;
    }
    strr
}

fn main() -> io::Result<()> {
    let mut tasks: Vec<Task> = vec![];
    loop {
        list_tasks(&tasks);

        let option = readline();
        _ = match option.as_str() {
            "1" => {
                println!("Enter new task name: ");
                let name = readline();
                add_task(&mut tasks, name);
            }
            "2" => {
                println!("Enter task to complete: ");
                let level: i32 = readline().parse::<i32>().unwrap();
                complete_task(&mut tasks, level);
            }
            _ => break,
        };
    }
    Ok(())
}

fn add_task(tasks: &mut Vec<Task>, name: String) -> io::Result<()> {
    tasks.push(Task {
        name: name,
        is_completed: false,
    });
    Ok(())
}

fn list_tasks(tasks: &Vec<Task>) {
    for _ in 0..50 {
        println!("\n");
    }
    println!("Tasks List: ");
    for task in tasks {
        println!("Name: {}", task.name);
        println!("Is Completed: {}", task.is_completed);
    }
    println!(
        "Choose the following options:
1. Add tasks
2. Complete tasks
3. Exit"
    );
}

fn complete_task(tasks: &mut Vec<Task>, level: i32) -> io::Result<()> {
    tasks[level as usize].is_completed = true;
    Ok(())
}
