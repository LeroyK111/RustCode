/*
专用构建rs脚本
*/



/*
扁平化目录结构: 此规则的唯一例外是如果您有文件build.rs。在这种情况下，最好保留cargo的标准布局。

my_package/
    my_package.rs
    Cargo.toml
    README.md

# Cargo.toml   
[lib]
path = "./my_package.rs"
*/


