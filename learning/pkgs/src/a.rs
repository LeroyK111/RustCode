/*
todo 这里测试的是a模块引入pubVisibleModifier模块.

! pub(in crate::a)
详情见learning\pkgs\Horizontal_reference
*/


#[path = "./b.rs"] mod b; // 平级目录申明模块, 第一种方法
use b::restricted_function; // 通过模块访问
// use crate::b::restricted_function; // 这里也会造成失败, 必须有文件夹
// use super::b::restricted_function; // 这里也会造成失败, 必须有文件夹


pub fn public_func() {
    println!("This function is public in `a`.");
    // b::restricted_function(); // ✅ 可以访问
    restricted_function(); // ✅ 可以访问
}


