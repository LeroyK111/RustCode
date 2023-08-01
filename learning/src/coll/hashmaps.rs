use std::collections::HashMap;

pub fn hashmaps() {
    let mut scores = HashMap::new();
    // 写入值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // 插入hash
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    // field_name， field_name 立刻失效
    map.insert(field_name, field_value);

    // 覆盖更新hash
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // 检查更新，没有才增加
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // 根据旧值更新新值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
