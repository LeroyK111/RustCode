/*
utils.rs：放置常用的工具函数，如复制，哈希生成等。
*/


// src/utils.rs文件中的suggester函数用于过滤用户输入的命令，如果用户没有输入命令，则显示全部命令供用户选择。代码如下：

pub fn suggester(val: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let suggestions = [
        "get",
        "set",
        "delete",
        "list",
        "search",
        "help",
        "exit",
    ];

    let val_lower = val.to_lowercase();

    Ok(suggestions
        .iter()
        .filter(|s| s.to_lowercase().contains(&val_lower))
        .map(|s| String::from(*s))
        .collect())
}


pub fn random_hash()->String{
    let mut rng = rand::thread_rng();
    let rand_string: String = std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric) as char)
        .take(6)
        .collect();
    rand_string
}

pub fn get_table() -> tabled::builder::Builder {
    let mut builder = Builder::default();
    builder.set_header(
        vec![
            "Key".to_string(),
            "Value".to_string(),
            "Hash".to_string(),
            "Created At".to_string(),
        ]
    );
    builder
}