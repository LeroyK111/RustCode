// 首先定义一个事件枚举：
#[derive(Clone, Debug, PartialEq, Eq, Hash, Display, EnumString)]
pub enum Event {
    Dummy,
    TestEvent,
}

// 定义一个类型别名Payload：
pub type Payload = Vec<u8>;
// Payload被定义为Vec<u8>的类型别名，表示与事件关联的数据。

// 定义一个Trait Handler：
pub trait Handler: Send + Sync {
    fn handle_event(&self, event: Event, payload: Payload);
}

// Handler是任何事件处理程序都必须实现的trait。它需要一个方法handle_event，该方法接受Event和Payload。

// 由于事件循环在自己的线程中运行，并且可能跨不同线程与多个Handler程序交互，因此Handler程序可能需要在线程之间移动。通过标记Send Trait，可以确保Handler实现程序可以跨线程边界传输，从而使事件循环在多线程上下文中变得安全和健壮。

// Handler程序存储在一个共享的Arc<Mutex<HashMap<Event, Vec<Arc<dyn Handler>>>>>中。Arc(原子引用计数)允许多个线程共享处理程序的所有权，而无需克隆它们。通过标记Sync Trait，可以确保多个线程可以安全地保存对同一Handler程序的引用。这意味着对Handler程序状态的任何读访问都是线程安全的。

// 定义一个结构体Listener：
#[derive(Clone)]
pub struct Listener {
    pub event: Event,
    pub handler: Arc<dyn Handler>,
}
// Listener结构体将事件绑定到实现handler的处理程序，从而允许为每个事件添加多个处理程序。

// 定义一个Dispatcher结构体：
pub struct Dispatcher {
    tx: Sender<(Event, Payload)>,
    rx: Receiver<(Event, Payload)>,
    // 事件注册表
    registry: Arc<Mutex<HashMap<Event, Vec<Arc<dyn Handler>>>>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Dispatcher {
            tx,
            rx,
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 向注册表中注册事件及其对应的处理程序
    pub fn register_handler(&mut self, event: Event, handler: Arc<dyn Handler>) {
        let mut registry = self.registry.lock().unwrap();
        registry.entry(event).or_insert_with(Vec::new).push(handler);
    }

    // 向通道发送事件
    pub fn trigger_event(&self, event: Event, payload: Payload) {
        self.tx.send((event, payload)).unwrap();
    }

    // 接收通道中的事件并进行处理
    pub fn start(&self) {
        let registry = Arc::clone(&self.registry);
        let rx = self.rx.clone();

        thread::spawn(move || loop {
            if let Ok((event, payload)) = rx.recv() {
                let registry = registry.lock().unwrap();
                if let Some(handlers) = registry.get(&event) {
                    for handler in handlers {
                        handler.handle_event(event.clone(), payload.clone());
                    }
                }
            }
        });
    }
}
// Dispatcher保存事件通道的发送方和接收方以及线程安全的Handler实现集合。

// 我们定义多个事件处理器：
pub struct TestEventHandler;

impl Handler for TestEventHandler {
    fn handle_event(&self, event: Event, payload: Payload) {
        let data = String::from_utf8(payload).unwrap();
        let message = format!("{} => {}", event, data);

        println!("TestEvent: {}", message);
    }
}

pub struct DBTestEventHandler;

impl Handler for DBTestEventHandler {
    fn handle_event(&self, event: Event, payload: Payload) {
        let data = String::from_utf8(payload).unwrap();
        let message = format!("{} => {}", event, data);

        // 将数据持久化到db中
        println!("Data: {} saved on DB!", message);
    }
}

// 在main函数中写入以下代码：
fn main() {
    let mut event_loop = Dispatcher::new();

    event_loop.register_handler(Event::TestEvent, Arc::new(TestEventHandler));
    event_loop.register_handler(Event::TestEvent, Arc::new(DBTestEventHandler));

    // 启动事件循环
    event_loop.start();

    loop {
        println!("Give me some input, type 'exit' to quit");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Error during input");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut split = input.split_whitespace();
        let name_data = (
            split.next().unwrap_or_default().to_string(),
            split.next().unwrap_or_default().to_string(),
        );

        let event = Event::from_str(&name_data.0).unwrap_or_else(|_| Event::Dummy);
        event_loop.trigger_event(event, name_data.1.as_bytes().to_vec());
    }
}
