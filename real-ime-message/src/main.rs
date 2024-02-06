
/*
events：存储每个主题的事件。
subscribers：跟踪每个主题的订阅者。
*/
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use futures_util::{SinkExt, StreamExt};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};
use warp::{filters::ws::Message, Filter};

type Topic = String;
type Event = String;
type WsSender = UnboundedSender<warp::ws::Message>;

struct Broker {
    events: Arc<RwLock<HashMap<Topic, VecDeque<Event>>>>,
    subscribers: Arc<RwLock<HashMap<Topic, Vec<WsSender>>>>,
}

// 创建实例化
impl Broker {
    fn new() -> Self {
        Broker {
            events: Arc::new(RwLock::new(HashMap::new())),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // 定义发布事件的方法produce：
    async fn produce(&self, topic: Topic, event: Event) {
    let mut events = self.events.write().await;
    events
        .entry(topic.clone())
        .or_default()
        .push_back(event.clone());

    // 异步通知所有订阅者
    let subscribers_list;
    {
        let subscribers = self.subscribers.read().await;
        subscribers_list = subscribers.get(&topic).cloned().unwrap_or_default();
    }

    for ws_sender in subscribers_list {
        // 将事件发送到WebSocket客户端
        let _ = ws_sender.send(warp::ws::Message::text(event.clone()));
        }
    }


    // 定义subscribe方法，来管理新的订阅：
      pub async fn subscribe(&self, topic: Topic, socket: warp::ws::WebSocket) {
        let (ws_sender, mut ws_receiver) = socket.split();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        {
            let mut subs = self.subscribers.write().await;
            subs.entry(topic).or_default().push(tx);
        }

        tokio::task::spawn(async move {
            while let Some(result) = ws_receiver.next().await {
                match result {
                    Ok(message) => {
                        // 处理有效的消息
                        if message.is_text() {
                            println!(
                                "Received message from client: {}",
                                message.to_str().unwrap()
                            );
                        }
                    }
                    Err(e) => {
                        // 处理错误
                        eprintln!("WebSocket error: {:?}", e);
                        break;
                    }
                }
            }
            println!("WebSocket connection closed");
        });

        tokio::task::spawn(async move {
            let mut sender = ws_sender;

            while let Some(msg) = rx.recv().await {
                let _ = sender.send(msg).await;
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let broker = Arc::new(Broker::new());
    let broker_clone1 = Arc::clone(&broker);
    let broker_clone2 = Arc::clone(&broker);

    let produce = warp::path!("produce" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&broker_clone1)))
        .and_then(
            move |topic: String, event: Event, broker_clone2: Arc<Broker>| async move {
                broker_clone2.produce(topic, event).await;
                Ok::<_, warp::Rejection>(warp::reply())
            },
        );

    let subscribe = warp::path!("subscribe" / String).and(warp::ws()).map(
        move |topic: String, ws: warp::ws::Ws| {
            let broker_clone3 = Arc::clone(&broker_clone2);
            ws.on_upgrade(move |socket| async move {
                broker_clone3.subscribe(topic.clone(), socket).await;
            })
        },
    );

    let routes = produce.or(subscribe);

    println!("Broker server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
