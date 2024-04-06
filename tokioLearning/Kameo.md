# Kameo: 基于 Tokio 构建的异步 Actor

使用宏的方式（也有不用宏的方式）定义一个  actor

```rust
// Define the actor state
#[derive(Actor)]
struct Counter {
    count: i64,
}

// Define messages
#[actor]
impl Counter {
    #[message]
    fn inc(&mut self, amount: u32) -> Result<i64, Infallible> {
        self.count += amount as i64;
        Ok(self.count)
    }
}
```
执行方式

```rust
use kameo::{Spawn, ActorRef};

let counter_ref: ActorRef<Counter> = Counter { count: 0 }.spawn();

let count = counter_ref.send(Inc(42)).await?;
println!("Count is {count}");
```