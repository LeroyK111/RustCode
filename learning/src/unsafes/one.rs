/*
普通 Rust：
编译器检查相应的安全规则。

unsafe 操作：
有一些条件编译器无法验证，
程序员必须确保它们成立。

| 操作          | 你需要额外负责什么          |
| ----------- | ------------------ |
| 解引用裸指针      | 指向的内存是否适合这次读取或写入   |
| 调用不安全函数     | 是否满足函数文档规定的条件      |
| 读写可变静态变量    | 是否存在不合法的并发访问或别名访问  |
| 实现不安全 trait | 是否满足该 trait 的安全契约  |
| 读取 union 字段 | 这片存储中的数据能否按该字段类型解释 |
*/

pub fn test() {
    let mut number = 5;

    // 创建裸指针，不需要 unsafe
    let readPointer: *const i32 = &raw const number;
    let writePointer: *mut i32 = &raw mut number;

    println!("指向的地址：{readPointer:p}");

    // &raw const number // 创建指向 number 的裸指针

    // *readPointer      // 读取指针指向的数据

    // *writePointer = 10 // 修改指针指向的数据

    // SAFETY:
    // number 仍然存活，已经初始化；
    // 指针来自这个局部变量；
    // 此处没有并发访问，也没有冲突的引用访问。
    unsafe {
        println!("修改前：{}", *readPointer);

        *writePointer = 10;

        println!("修改后：{}", *readPointer);
    }

    println!("number = {number}");
}

pub fn test2() {
    /// # Safety
    ///
    /// 调用者必须保证：
    /// pointer 指向有效、对齐、已初始化的 i32；
    /// 读取期间目标仍然存活，且没有冲突的访问。
    unsafe fn readNumber(pointer: *const i32) -> i32 {
        // SAFETY: 由调用者保证上述条件。
        unsafe { *pointer }
    }

    let number = 5;
    let pointer = &raw const number;

    // SAFETY: 指针来自仍然存活的局部变量，
    // 此处仅进行读取，没有冲突访问。
    let value = unsafe { readNumber(pointer) };

    /*
    | 写法                             | 表达的意思              |
    | ------------------------------ | ------------------ |
    | unsafe fn readNumber(...)      | 调用这个函数，需要调用者满足额外条件 |
    | unsafe { readNumber(pointer) } | 调用者在这里承担满足条件的责任    |
    | 函数内部的 unsafe { *pointer }      | 在这里执行需要额外保证的操作     |
        */
    println!("{value}");
}

pub fn test3() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6];

    let (left, right) = numbers.split_at_mut(3);

    /*
    原始数据：[1, 2, 3,   4, 5, 6]
                        ↑
                        索引 3

        left： [1, 2, 3]
        right：[4, 5, 6]
    */

    left[0] = 10;
    right[0] = 40;

    println!("{numbers:?}");

    // !如果自己实现这个功能, 则会报错.
    // fn split(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     assert!(mid <= values.len());

    //     (&mut values[..mid], &mut values[mid..])
    // }

    use std::slice;

    // 重写方法
    fn split(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let pointer = values.as_mut_ptr();

        assert!(mid <= len);

        // SAFETY:
        // 指针和长度来自一个有效的可变切片；
        // mid 已检查，两个范围都在原切片内且互不重叠；
        // 返回的切片继续借用原数据。
        unsafe {
            let left = slice::from_raw_parts_mut(pointer, mid);

            let right = slice::from_raw_parts_mut(pointer.add(mid), len - mid);

            (left, right)
        }
    }

    fn main() {
        let mut numbers = vec![1, 2, 3, 4, 5, 6];

        let (left, right) = split(&mut numbers, 3);

        left[0] = 10;
        right[0] = 40;

        println!("{numbers:?}");
    }

    main()
}

// 这里就是给c语言的func
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("从 C 调用了 Rust");
}

pub fn test4() {
    /*
    extern "C"
    → 使用 C 的调用约定

    fn abs(...)
        → 声明外部函数的名字和签名
        → 不是在这里实现函数

    unsafe
        → 外部声明、调用的正确性需要人工保证
    */
    unsafe extern "C" {
        // 这里放入 c 申明的方法和对象
        fn abs(input: i32) -> i32;
    }

    // SAFETY: 此处传入 -3，结果 3 可以表示。
    let value = unsafe { abs(-3) };

    println!("{value}");
}

pub fn test5() {
    // 只读静态变量
    static MESSAGE: &str = "Hello";
    println!("{MESSAGE}");

    // 可变静态变量
    static mut COUNTER: u32 = 0;

    // SAFETY: 本例只有 main 线程访问 COUNTER，
    // 没有创建指向它的引用。
    unsafe {
        COUNTER += 3;

        let value = *(&raw const COUNTER);

        println!("COUNTER = {value}");
    }
}

pub fn test6() {
    /// # Safety
    /// 实现者必须保证 test() 返回的指针：
    /// 指向一个已初始化的 i32，且在整个程序运行期间有效。
    unsafe trait Foo {
        // 这里就是申明这是一个契约
        fn test() -> *const i32;
    }

    // SAFETY：返回的指针指向不可变静态变量 NUMBER，
    // 它在整个程序运行期间保持有效。
    unsafe impl Foo for *const i32 {
        fn test() -> *const i32 {
            static NUMBER: i32 = -15;

            &raw const NUMBER
        }
    }

    let pointer = <*const i32 as Foo>::test();

    // SAFETY：这个实现返回有效、对齐且已初始化的静态 i32 指针。
    unsafe {
        println!("{}", *pointer); // -15
    }
}

pub fn test7() {
    /*
        struct：
    每个字段分别有自己的存储。

    union：
    多个字段共享同一片存储。
        */
    union Data {
        integer: u32,
        decimal: f32,
    }

    let data = Data { integer: 42 };

    // SAFETY: 此处读取的是刚写入的 u32 字段。
    let value = unsafe { data.integer };

    println!("{value}");
}

/*
rustup toolchain install nightly
rustup +nightly component add miri

cargo +nightly miri run
cargo +nightly miri test

检查unsafe的工具
*/
