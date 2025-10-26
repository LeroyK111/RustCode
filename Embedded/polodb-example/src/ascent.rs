/*
! Ascent：在 Rust 中嵌入的逻辑编程语言

开发者 s-arash 发布了 Ascent，一种嵌入在 Rust 中的逻辑编程语言。Ascent 类似于 Datalog，通过宏的形式在 Rust 中嵌入，为开发者提供了简洁而强大的逻辑编程解决方案。

Ascent 使开发者能够轻松解决图论、路径计算等问题，例如计算图中的连接节点、寻找最短路径等。
*/


use ascent::ascent;

ascent! {
    // 定义两个关系：边和路径
    relation edge(i32, i32);
    relation path(i32, i32);

    // 定义路径的规则：如果有边连接两个节点，则它们之间存在路径
    path(x, y) <-- edge(x, y);
    // 定义路径的规则：如果有边连接节点 x 和节点 y，并且存在路径从节点 y 到节点 z，则存在路径从节点 x 到节点 z
    path(x, z) <-- edge(x, y), path(y, z);
}

fn main() {
    let mut prog = AscentProgram::default();
    // 设置边的关系数据
    prog.edge = vec![(1, 2), (2, 3)];
    // 执行逻辑编程操作
    prog.run();
    // 打印路径的结果: path: [(1, 2), (2, 3), (1, 3)]
    println!("path: {:?}", prog.path);
}