/*
handler.rs：本质上充当中间件
*/

use tabled::settings::Style;

pub async fn add(){
    bunt::println!("执行add命令...");
    let key = inquire::Text::new("输入键: ").with_help_message("输入任意标识符").prompt().unwrap();
    let value = inquire::Text::new("输入值: ").with_help_message("输入任意值").prompt().unwrap();
    let hash = super::utils::random_hash();
    super::db::add(key.clone(), value.clone(), hash).await;
    bunt::println!("添加实体成功: {$green}{}{/$}", key);
    bunt::println!("值: {$yellow}{}{/$}", value);
}

pub async fn list(){
    bunt::println!("执行list命令...");
    let entries = super::db::list().await.unwrap();
    let mut builder = super::utils::get_table();
    for entry in entries {
        builder.push_record(vec![
            entry.key,
            entry.value,
            entry.hash,
            // 格式化日期时间为人类可读的格式
            chrono::DateTime::parse_from_rfc3339(&entry.created_at)
                .unwrap()
                .format("%a %b %e %T %Y")
                .to_string(),
        ]);
    }
    let table = builder.build().with(Style::rounded()).to_string();
    bunt::println!("{}", table);
}

pub async fn delete(){
    bunt::println!("执行delete命令...");
    let key = inquire::Text::new("输入键: ").with_help_message("输入任意标识符").prompt().unwrap();
    super::db::delete(key.clone()).await.unwrap();
    bunt::println!("删除实体成功: {$red}{}{/$}", key);
}

pub async fn get(){
    bunt::println!("执行get命令...");
    let key = inquire::Text::new("输入键: ").with_help_message("输入任意标识符").prompt().unwrap();
    let entry = super::db::get(key.clone()).await.unwrap();
    bunt::println!("实体: {$green}{}{/$}", entry.key);
    bunt::println!("值: {$yellow}{}{/$}", entry.value);
}

pub async fn search(){
    bunt::println!("执行搜索命令...");
    let keys = super::db::list_keys().await.unwrap();
    let key = inquire::Select::new("Select Key: ", keys).with_page_size(10).prompt().unwrap();
    let entry = super::db::get(key.clone()).await.unwrap();
    bunt::println!("实体: {$green}{}{/$}", entry.key);
    bunt::println!("值: {$yellow}{}{/$}", entry.value);
}