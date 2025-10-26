usehash_cons::{HcTable,Hc};

#[derive(Hash,PartialEq,Eq,Debug)]
structNode{
    value:i32,
    left:Option<Hc<Node>>,
    right:Option<Hc<Node>>,
}


// 在这个例子中，node2和node3是相同的，并且共享相同的内存空间，这要归功于hash_cons库。
fnmain() {
    lettable=HcTable::new();

    letnode1=table.hashcons(Node{value:1,left:None,right:None});
    letnode2=table.hashcons(Node{value:2,left:Some(node1.clone()),right:None});
    letnode3=table.hashcons(Node{value:2,left:Some(node1.clone()),right:None});

    //由于哈希压缩，Node2和node3应该是相同的
    assert!(std::ptr::eq(node2.as_ref(),node3.as_ref()));
    println!("{:?}",node2);
}

// Rust中的hash_cons库是一个强大的工具，用于在处理复杂、重复数据结构的应用程序中优化内存。通过利用哈希压缩，可以确保共享相同的结构，从而显著节省内存并提高性能。