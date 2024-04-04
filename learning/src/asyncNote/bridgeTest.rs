/*
如何桥接Rust异步和同步代码
[dependencies]
futures = {version = "0.3.29", features = ["executor"]}
*/





lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("tokio")
        .enable_all()
        .build()
        .unwrap();
}

impl Sequencer for PlainSequencer {
    fn generate(&self) -> Vec<i32> {
        // self.generate_async().await
        // RUNTIME.block_on(async { self.generate_async().await })
        futures::executor::block_on(async {
            self.generate_async().await
        })
    }
}

#[tokio::main]
pub async fn main() {
    let sequencer = PlainSequencer {
        bound: 3
    };
    let vec = sequencer.generate();
    println!("vec: {:?}", vec);
}