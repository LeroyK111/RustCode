fn main() -> Result<(), db_type::Error> {
    // 在内存中创建数据库
    let db = Builder::new().create_in_memory(&MODELS)?;

    // 插入数据(打开读写事务)
    let rw = db.rw_transaction().unwrap();
    rw.insert(Item {
        id: 1,
        name: "red".to_string(),
    })?;
    rw.insert(Item {
        id: 2,
        name: "green".to_string(),
    })?;
    rw.insert(Item {
        id: 3,
        name: "blue".to_string(),
    })?;
    rw.commit()?;

    // 打开只读事务
    let r = db.r_transaction()?;
    // 检索id=3的数据
    let retrieve_data: Item = r.get().primary(3_u32)?.unwrap();
    println!("data id='3': {:?}", retrieve_data);
    // 迭代名称以“red”开头的项
    for item in r.scan().secondary::<Item>(ItemKey::name)?.start_with("red") {
        println!("data name=\"red\": {:?}", item);
    }

    // Remove data (open a read-write transaction)
    let rw = db.rw_transaction()?;
    rw.remove(retrieve_data)?;
    rw.commit()?;

    Ok(())
}