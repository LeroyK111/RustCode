

mod abfolder;


// 这里就是self最常见的用法了, 用来分离mod嵌套的调用
mod a {
    pub fn hello() {
        println!("Hello from a");
    }

    pub fn world() {
        println!("World from a");
    }

    use self::inner::greet; // 使用 use self 来引用模块内部的 inner::greet

    pub mod inner {
        pub fn greet() {
            println!("Hello from inner module");
        }
    }

    pub fn greet_from_a() {
        hello(); // 直接调用 hello，不需要 `self::hello()`
        world(); // 直接调用 world，不需要 `self::world()`
        greet(); // 直接调用 greet，不需要 `self::inner::greet()`
    }
    /*
    mod a {
    pub fn hello() {
        println!("Hello from module a");
    }

    pub fn world() {
        println!("World from module a");
        }
    }


    ! 这是另一种用法
    mod b {
        use super::a::hello; // 引入模块 a 的 hello 函数
        use self::world;     // 引入当前模块 b 的 world 函数

        pub fn world() {
            println!("World from module b");
        }

        pub fn greet() {
            hello();
            world();
        }
    }
    */
}

// extern crate rand;  // !显式引入 rand crate;
// !在 Rust 2018 版本中，extern crate 语法已经不再是必须的，Cargo 会自动解析并引入在 Cargo.toml 中声明的依赖。所以你可以直接通过 use 来引入库中的功能。


fn main() {
    // d模块的f2函数, 间接调用c模块的function函数
    abfolder::d::f2();
}
