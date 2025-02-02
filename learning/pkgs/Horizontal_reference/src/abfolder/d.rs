// todo super相对引用, 使用上级目录确定同级模块文件的位置
// use super::c::function;

// pub fn f2() {
//     function();
// }


// todo crate根目录出发, 使用crate::确定同级模块文件的位置
// use crate::abfolder::c::function;

// pub fn f2() {
//     function();
// }


// ! self 这里首要问题就是 cargo默认就会在去c文件夹下找对应的函数 
// mod c;
// use self::c::function;
// pub fn f2() {
//     function();
// }

// todo 这里我们使用申明mod引用, 确保cargo不会去c文件夹下找对应的函数
#[path = "../abfolder/c.rs"] mod c;
use self::c::function; // todo 使用self也是确保调用的是c mod中的function函数
pub fn f2() {
    // self的用法
    function();
}




// !可以看到无法直接引用 mod 只要你使用了mod, cargo默认就会在去c文件夹下找对应的函数
/*
error[E0583]: file not found for module `c`
 --> src\abfolder\d.rs:9:1
  |
9 | mod c;
  | ^^^^^^
  |
  = help: to create the module `c`, create file "src\abfolder\d\c.rs" or "src\abfolder\d\c\mod.rs"
  = note: if there is a `mod c` elsewhere in the crate already, import it with `use crate::...` instead

error[E0425]: cannot find function `function` in module `c`
  --> src\abfolder\d.rs:12:8
   |
12 |     c::function();
   |        ^^^^^^^^ not found in `c`
   |
help: consider importing this function

*/
// mod c;
// pub fn f2() {

//     c::function();
// }



