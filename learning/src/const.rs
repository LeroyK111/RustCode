/*
! Rust团队发布了Rust 1.83.0版本，这个版本对运行在const上下文中的代码所能做的事情进行了几个大的功能扩展。这指的是编译器编译的所有代码必须在编译时求值：const和static设置初始值、数组长度、枚举值、const泛型参数以及可从这些上下文中调用的const函数(const fn)。
*/


// ?Const引用静态量
static S: i32 = 25;
const C: &i32 = &S;

static mut S: i32 = 0;
const C1: i32 = unsafe { S };
// error: 常量访问可变全局内存
const C2: &i32 = unsafe { &S };
// error: 对‘const’中可变内存的引用

static mut S: i32 = 64;
const C: *mut i32 = &raw mut S;



// ?可变引用和指针
const fn inc(x: &mut i32) {
    *x += 1;
}

const C: i32 = {
    let mut c = 41;
    inc(&mut c);
    c
};



use std::cell::UnsafeCell;

const C: i32 = {
    let c = UnsafeCell::new(41);
    unsafe { *c.get() += 1 };
    c.into_inner()
};

const C: &mut i32 = &mut 4;
// error[E0764]: 可变引用不允许出现在常量的最终值中