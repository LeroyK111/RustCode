#![allow(non_snake_case)] // 全局禁用模块命名警告
#![allow(dead_code)] // 全局禁用未使用函数的警告

/*
todo: pkgs用来讲解rust 模块系统

基本原则：

mod 定义模块（模块名与文件名或文件夹名相关）
pub 使项目中的元素可以被外部访问
use 用于简化路径，方便访问
模块可以嵌套，也可以拆分到不同的文件或目录
*/

mod my_module {
    fn test_module() {
        println!("no module pub")
    }

    pub fn test_module1() {
        println!("使用 pub 暴漏函数和对象")
    }

    pub fn test_module2() {
        println!("多个函数")
    }
}
// 支持多个引用
// use my_module::test_module1;  // 导入函数，简化调用, 这里是引出第一个对象
// use my_module::test_module1 as tm1;  // 支持重命名调用
// use my_module::test_module2;  // 导入函数，简化调用, 这里是引出第二个对象
// use my_module::{test_module1, test_module2};  // 导入多个函数，简化调用
// use my_module::{test_module1 as tm1, test_module2}; // 多个重名调用
// use my_module::*; // 导入所有pub函数，简化调用, 但是不用使用as 了

// 引用外部的library
// use myLibrary::add;

// 同级目录跨文件调用 module
mod myModule;
// 申明模块后, 为了更简单, 可以使用use,
// use myModule::myMoudle1;

// 测试修饰符
mod pubVisibleModifier;

// 调用传统方法sonModule
mod sonModule;
// 现代方法
mod sonModule1;

// 测试结构体导出
mod my_module1 {
    // 结构体
    pub struct User {
        pub name: String, // `pub` 使得外部可访问
        age: u32,         // 没有 `pub`，外部无法访问
    }

    // 枚举
    #[derive(Debug)] // #[derive(Debug)] 快速实现Debug trait
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    // 实现结构体, 构造方法, 这里会在readme.md 详细讲解
    impl User {
        pub fn new(name: &str, age: u32) -> Self {
            Self {
                name: name.to_string(),
                age,
            }
        }
    }
}

// 修饰符 pub(in path)（限定路径访问）
mod a;
// mod b;

fn main() {
    // 导出结构体
    // let user = my_module1::User::new("Alice", 30);
    // println!("user: {:?}", user.name);
    // println!("user: {:?}", user.age); //  private field

    // 导出枚举
    // let color = my_module1::Color::Red;
    // println!("{:?}", color); // 输出：Red

    // *直接引用当前文件下的module
    // my_module::test_module();
    // my_module::test_module1(); // 这段代码可以正常编译通过，因为test_module1是pub的

    // 使用use导入模块
    // test_module1();
    // test_module2();

    // 重命名调用
    // tm1()

    // 调用外部的library
    // let result = add(10, 20);
    // println!("result: {}", result);

    // 同级文件调用 mod 链式调用
    // myModule::test_module3();
    // myModule::myMoudle1::test_module1();
    // 同级文件调用 mod+use 调用
    // myMoudle1::test_module1();

    // 跨文件夹调用 两种风格
    // sonModule::sub_module::say_hello(); // 传统的
    // sonModule1::son::say(); // 现代的

    // pub(crate) 理解
    // use pubVisibleModifier::{test, public_function, internal_function};
    // test(); // 允许任何模块访问
    // public_function(); // 仅限当前 crate 内可见
    // internal_function(); // 同一个cargo.toml
    // !如果没有mod, 那外界引用必须使用use
    // pubVisibleModifier::test();

    // pub(super)理解
    // pubVisibleModifier::super_test(); //  这个只是检测pub(super) 裸用
    // !正式使用嵌套mod & pub(super)
    // pubVisibleModifier::parent::test();
    // ❌错误！main 不能直接访问
    // pubVisibleModifier::parent::child::parent_only_function();

    // pub(in path)（限定路径访问）
    // pubVisibleModifier::in_path_test();
    // pubVisibleModifier::in_path();
    // 限定路径访问, 需要使用指定mod/可以是任何位置的crate、super, mod
    // todo 📌 pub(in crate::a) 适用场景
    // *你想让一个函数只在 a 及其子模块中可见，但不希望它暴露给 main.rs 或其他模块。
    // *比 pub(crate) 更精细的控制，确保 API 不会在整个 crate 内部滥用。
    // 这里我们使用间接访问模块, 通过a模块访问b模块
    a::public_func();
    // 直接访问b,则直接报错
    // b::restricted_function(); // ❌错误！     error[E0742]: visibilities can only be restricted to ancestor modules


}
