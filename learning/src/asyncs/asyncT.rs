use std::pin::Pin;
use std::task::{Context, Poll};

// 定义
pub trait Future {
    /*
        | 部分                   | 含义                          |
    | -------------------- | --------------------------- |
    | type Output          | Future 完成后产生什么类型的值          |
    | poll(...)            | 推进计算，并检查是否已经完成              |
    | self: Pin<&mut Self> | 用一种受约束的可变引用访问 Future，下一步详细讲 |
    | cx: &mut Context<'_> | 提供当前任务的上下文，里面可以取得唤醒器        |
    | Poll<Self::Output>   | 返回“未完成”或“已完成及其输出”           |
        */
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
