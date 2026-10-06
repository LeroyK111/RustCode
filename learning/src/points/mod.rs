/*
在 Rust 中，由于所有权和借用的概念，引用和智能指针之间还有一个额外区别：引用只会借用数据，而智能指针在很多情况下会拥有它们所指向的数据。
*/

pub mod BoxTest;
pub mod DerefTest;
pub mod DropTest;
pub mod RcTest;
pub mod RefCellTest;
pub mod oomTest;
