use std::time::Duration;

use tokio::sync::{mpsc, oneshot};

async fn cooking_stand(
    product: char,
    mut waiters: tokio::sync::mpsc::Receiver<oneshot::Sender<char>>,
) {
    while let Some(waiter) = waiters.recv().await {
        waiter.send(product).unwrap();
    }
}

async fn table(number: u8, mut waiters: tokio::sync::mpsc::Receiver<oneshot::Receiver<char>>) {
    while let Some(waiter) = waiters.recv().await {
        let food = waiter.await.unwrap();
        println!("Got {} at table {}", food, number);
    }
}

#[tokio::main]
async fn main() {
    // 经理分配服务员到烹饪台
    let (stand_salad_tx, stand_salad_rx) = mpsc::channel::<oneshot::Sender<char>>(100);
    let (stand_pizza_tx, stand_pizza_rx) = mpsc::channel::<oneshot::Sender<char>>(100);
    let (stand_burger_tx, stand_burger_rx) = mpsc::channel::<oneshot::Sender<char>>(100);

    // 搭建烹饪台
    tokio::spawn(cooking_stand('🥗', stand_salad_rx));
    tokio::spawn(cooking_stand('🍕', stand_pizza_rx));
    tokio::spawn(cooking_stand('🍔', stand_burger_rx));

    // 经理分配服务员到餐桌
    let mut tables: Vec<tokio::sync::mpsc::Sender<oneshot::Receiver<char>>> = Vec::new();

    for number in 1..=4 {
        let (table_tx, table_rx) = mpsc::channel::<oneshot::Receiver<char>>(100);
        tables.push(table_tx);
        tokio::spawn(table(number, table_rx));
    }

    // 创建服务员
    let (waiter_tx, waiter_rx) = oneshot::channel::<char>();
    // 分配到沙拉烹饪台
    stand_salad_tx.send(waiter_tx).await.unwrap();
    // 让他把食物送到1号桌
    tables.first().unwrap().send(waiter_rx).await.unwrap();

    // 创建服务员
    let (waiter_tx, waiter_rx) = oneshot::channel::<char>();
    // 分配到披萨烹饪台
    stand_pizza_tx.send(waiter_tx).await.unwrap();
    // 让他把食物送到2号桌
    tables.get(1).unwrap().send(waiter_rx).await.unwrap();

    // 创建服务员
    let (waiter_tx, waiter_rx) = oneshot::channel::<char>();
    // 分配到披萨烹饪台
    stand_burger_tx.send(waiter_tx).await.unwrap();
    // 让他把食物送到3号桌
    tables.get(2).unwrap().send(waiter_rx).await.unwrap();

    tokio::time::sleep(Duration::from_millis(1000)).await;
}