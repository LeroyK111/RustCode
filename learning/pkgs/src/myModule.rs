
// #![allow(non_snake_case)]  // 只禁用模块命名警告
// #![allow(dead_code)]       // 只禁用未使用函数的警告

/*
todo: 这里我们使用一个同级目录下的文件

mod: 支持嵌套导入导出
其他函数or对象则不支持嵌套
*/


// 我们可以使用一个模块定义, 同样要使用pub
pub mod myMoudle1 {
    fn test_module(){
        println!("no module pub")
    }


    pub fn test_module1(){
        println!("pub mod myMoudle1")
    }

    #[allow(dead_code)] // 禁用这个函数的警告
    pub fn test_module2(){
        println!("多个函数")
    }

    pub mod myMoudle2 {
        pub fn test_module3(){
            println!("使用 pub 暴漏 mod")
        }
    }
}


pub fn test_module3(){
    println!("使用 pub 暴漏函数和对象")
}


fn test_module4(){
    println!("no module pub")
}