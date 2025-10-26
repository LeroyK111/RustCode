// src/a.rs


use super::b::restricted_function;

pub fn call_b() {
    restricted_function(); // 调用 b 中的函数
}
