pub fn function() {
    println!(r"learning\pkgs\Horizontal_reference\src\abfolder\c.rs");

    // 正常的字符串字面量会需要转义引号
    println!("This is a \"quoted\" string");
    
    // 使用原始字符串字面量，避免了转义
    println!(r#"This is a "quoted" string"#);

    // 普通字符串字面量需要手动转义换行符
    let normal_string = "This is line 1\nThis is line 2\nThis is line 3";
    println!("{}", normal_string);

    // 原始字符串字面量不需要转义换行符
    let raw_string = r"This is line 1
This is line 2
This is line 3";
    println!("{}", raw_string);

    let regex = r"\d{3}-\d{2}-\d{4}"; // 正则表达式不需要转义
    println!("Regex pattern: {}", regex);
}

