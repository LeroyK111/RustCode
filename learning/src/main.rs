// 导入库
use ferris_says::say;
use std::io::{self, stdout, BufWriter};

use std::any::{type_name, type_name_of_val, Any};
// 导入自己包，先声明同级目录下的rs文件
// 私有模块声明
mod game;
// 这是相对路径
use game::{g, test};
// 只引用一个函数
// use game::g;

// 共有模块声明
mod variablesVariability;
use variablesVariability::{constants, testType, test_mut, Shadowing};

// 绝对路径引入，这是根路径
// use crate::game::test;

// 测试中文包，不支持😄
// mod 数据类型;
// use 数据类型::{changeType;}

mod function;
use function::{
    if_main, plus_one, print_labeled_measurement, talk, testFor, testFor1, testLoop, testLoop1,
    testWhile,
};

mod advFunc;
use advFunc::{cloth, nextItem};

mod dataType;
use dataType::{array, bools, change_type, count, float, string, testArrayIndex, tuple};

// 测试所有权
mod ownership;
use ownership::{scope, scope1, scope2, scope3, scope4};
// 引用和借用
mod referenceBorrowing;
use referenceBorrowing::demo;

// 结构体
mod structure;
use structure::theWorld;

// 枚举体
mod enums;
use enums::{enums, enums1, enums2, enums3, enums4};

// 集合
mod coll;
use coll::hashmaps::hashmaps;
use coll::strings::strMain;
use coll::vectors::{call, call2, call3};

// 异常处理, 使用这种导入
mod errors {
    pub mod usePanic;
    pub mod useResult;
}

use errors::usePanic::testPanic;
use errors::useResult::{self, controlFile};

// 泛型,trait,生命周期, 使用这种导入
mod lifes;
use lifes::*;

// 智能指针
mod points;
use points::*;

// 多线程
mod process;

// 异步
mod asyncs;
use asyncs::{advA, asyncT, Fs, FA, STREAM};

// 对象编程
mod object;

// 模式
mod modes; 

// 高级应用
mod unsafes;

// 高级类型和闭包
mod funcAdv;

// 宏脚本
mod macros;
use macros::usages;


// 模式匹配中的专属范围
// Rust现在支持专属范围的模式匹配(a..b)，这增强了模式匹配并减少了对包含端点的单独常量的需求。
pub fn size_prefix(n: u32) -> &'static str {
    const K: u32 = 10u32.pow(3);
    const M: u32 = 10u32.pow(6);
    const G: u32 = 10u32.pow(9);
    match n {
        ..K => "",
        K..M => "k",
        M..G => "M",
        G.. => "G",
    }
}

fn test_guess() {
    // 提示
    println!("Please input your guess.");
    // 创建可变值， 这一行创建了一个可变变量，当前它绑定到一个新的 String 空实例上。
    let mut guess = String::new();

    // 这是不可变值
    // let apples = 5;
    // mut 表示变量可变，如果没有使用 apples1+=1 还有一个error提示
    // let mut apples1 = 6;
    // apples1 += 1;
    // println!("{apples}, {apples1}");

    // 引用io标准输入 句柄
    io::stdin()
        // 读取字符串 并引用 &mut guess 赋值
        .read_line(&mut guess)
        // 异常捕获, 如果不调异常捕获，程序可以执行，但是会报错提示
        .expect("捕获异常");

    // 打印字符串，模版语法
    println!("You guessed1: {guess}");

    /*
        io::stdin()
        // 读取字符串 并引用 &mut guess 赋值
        .read_line(&mut guess)
        // 异常捕获, 如果不调异常捕获，程序可以执行，但是会报错提示
        .expect("捕获异常");

    // 打印字符串，模版语法
    println!("You guessed2: {guess}");
    */

    // 将字符串转换为 i32（32 位整数）
    let z: i32 = match guess.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("错误代码：{}", e);
            // 主main中不支持 return
            -10
        }
    };

    // 设置两个常量
    let x = 5;
    let y = 10;
    // 模版语法
    println!("x = {x} and {y} + {z} = {}", y + z);
}

fn test_says() {
    /*
     这是彩蛋了
    */
    let stdout: std::io::Stdout = stdout();

    let message: &str = "Hello fellow Rustaceans!";
    let width: usize = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(message, width, &mut writer).unwrap();
}

// !主程序入口函数
fn main() {
    // 打印
    // println!("Hello, Rust!");

    // 测试 彩蛋库
    // test_says();

    // 测试猜测
    // test_guess();

    // 生成随机数
    // g();

    // *测试变量
    // test_mut();
    // 测试不可变量
    // constants()
    // 测试定义域
    // Shadowing();
    // 只能修改值，不能修改类型
    // testType()
    // 查看数据类型
    // println!("a: {}", type_name_of_val("a"));
    // let value = 123_i32;
    // if type_name_of_val(&value) == type_name::<i32>() {
    //     println!("类型名称相同");
    // }

    // 判断变量 主要是 is
    // let value: &dyn Any = &123_i32;
    // println!("是否为 i32：{}", value.is::<i32>());
    // if let Some(number) = value.downcast_ref::<i32>() {
    //     println!("取出的数字：{}", number);
    // }

    // *改变数据类型
    // change_type()
    // 标量类型
    // 浮点
    // float();
    // 计算
    // count();
    // 布尔值
    // bools();
    // 字符串
    // string();
    // 元组()
    // tuple();
    // 数组
    // array();
    // testArrayIndex();

    // 函数
    // print_labeled_measurement(32, '2');
    // talk(3);
    // let x = plus_one(2);
    // println!("{x}");
    // if_main();
    // testLoop();
    // testLoop1();
    // testWhile();
    // testFor();
    // testFor1();

    // 作用域函数，打印
    // scope();
    // 字符串类型
    // scope1();
    // scope2();
    // 字面量，深复制、克隆
    // scope3();
    // 函数和字面量
    // scope4();

    // 所有权：引用和借用
    // demo();

    // 结构体，有点像typescript的接口语法
    // theWorld();

    // 枚举
    // enums();
    // enums1();
    // enums2();
    // enums3();
    // enums4();

    // !集合

    /*
        & 借来看，&mut 借来改；
        * 顺引用，找到原值来；
        整数传副本，改了不影响；
        可变引用传，改到原身上。
    */

    // call();
    // call2();
    // call3();
    // strMain();
    // hashmaps();

    // ?错误处理
    // testPanic();
    // controlFile();
    // useResult::propagating();

    // todo 泛型,trait,生命周期
    // genericsTemp::test();
    // traitTemp::test();
    // 涉及到 dyn 分发,
    // traitTemp2::test();
    // lifeTemp::test();

    // !拆分代码和包引用等等, ./RustCode/showFolder/src/main.rs 进行学习
    // !测试宏: 我们在 ./restaurant/src/lib.rs 进行学习, 这里涉及到库的概念

    // ?例子: 构建 command line 工具: ./editCLI 中

    // todo 迭代器和闭包: 迭代器是 Rust 的零成本抽象 之一
    // cloth();
    // nextItem();
    /*
        | 抽象         | 你写的高层表达        | 编译器帮你做什么         |
    | ---------- | -------------- | ---------------- |
    | 泛型         | “同一段逻辑适用于多种类型” | 生成具体类型的版本        |
    | trait 静态分发 | “调用实现了这个行为的方法” | 提前确定具体实现         |
    | 普通借用       | “暂时使用别人的数据”    | 提前检查引用和借用是否合法    |
    | 迭代器与闭包     | “筛选、转换、累加”     | 通常优化成直接完成操作的底层代码 |
        */

    // !cargo 打包,发布等多功能工具: editCLI/cargo.toml
    // ?智能指针
    // BoxTest::test();
    // DerefTest::test();
    // DropTest::test();
    // RcTest::test();
    // RefCellTest::test();
    // RefCellTest::handleMsg();
    // RefCellTest::handlRcAndRefcell();
    // oomTest::bug();
    // oomTest::useWeak();

    /*
    | 类型或 trait  | 核心职责           |
    | ---------- | -------------- |
    | Box<T>     | 拥有并管理堆上的值      |
    | Rc<T>      | 提供共享所有权，维护引用计数 |
    | RefCell<T> | 管理内部值的运行时借用    |
    | Ref<T>     | 表示一次只读借用       borrow 返回值类型|
    | RefMut<T>  | 表示一次可变借用       borrow_mut 返回值类型|
    | Deref      | 支持像引用一样访问目标值   |
    | Drop       | 在对象销毁时执行收尾工作   |
    */



    // todo 并发编程
    // process::one::test();
    // process::one::test1();
    // process::two::test();
    // process::two::test1();
    // process::two::test2();
    // process::two::test3();
    // process::three::test();
    // process::three::test1();
    // process::four::test();
    // ! 异步编程
    // FA::test();
    // FA::test2();
    // FA::test3();
    // Fs::test();
    // Fs::test2();
    // Fs::test3();
    // Fs::test4();
    // Fs::test5();
    // advA::test();
    // advA::test2();
    // advA::test3();
    // STREAM::test();
    // STREAM::test2();
    // STREAM::test3();
    // asyncT::test();
    // asyncT::test2();
    // asyncT::test3();
    // asyncT::moveEle();
    // asyncT::test4();
    // asyncT::test5();
    // asyncT::test6();
    // asyncT::test7();

    // ?对象编程
    // object::base::test();
    // object::adv::test(); 
    // object::adv::test2();

    // todo 模式和匹配
    // modes::base::test();
    // modes::base::test2();
    // modes::base::test3();
    // modes::base::test4();
    // modes::base::test5();
    // modes::adv::test();
    // modes::adv::test2();
    // modes::adv::test3();
    // modes::adv::test4();
    // modes::adv::test5();
    // modes::adv::test6();
    // modes::adv::test7();

    // !高级特性, 裸指针等等
    // unsafes::one::test();
    // unsafes::one::test2();
    // unsafes::one::test3();
    // unsafes::one::test4();
    // unsafes::one::test5();
    // unsafes::one::test6();
    // unsafes::one::test7();

    // !高级trait
    // traitAdv::test();
    // traitAdv::test2();
    // traitAdv::test3();
    // traitAdv::test4();
    // traitAdv::test5();
    // traitAdv::test6();

    // !高级函数 和 闭包
    // funcAdv::bases::test();
    // funcAdv::bases::test2();
    // funcAdv::bases::test3();
    // funcAdv::bases::test4();
    // funcAdv::bases::test5();
    // funcAdv::bases::test6();
    // funcAdv::adv::test();
    // funcAdv::adv::test2();
    // funcAdv::adv::test3();
    // 三种函数申明
    // funcAdv::adv::mainx();
    // funcAdv::adv::main2x();
    // funcAdv::adv::main3x();
    // funcAdv::adv::test4();
    // funcAdv::adv::test5();
    // funcAdv::adv::test6();
    // funcAdv::adv::test7();
    // !宏 这里就是麻烦的地方
    // usages::test();
    // usages::test2();

    // ?例子: 多线程web server: \learning\webServer
    // todo: 2026 新特性学习: learning\src\2025..20xx
}
