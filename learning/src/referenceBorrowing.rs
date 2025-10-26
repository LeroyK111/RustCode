/*
todo 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。 与指针不同，引用确保指向某个特定类型的有效值。
*/

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 不支持返回引用值, 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// }

// 获取首字母
fn first_word(s: &String) -> usize {
    // 因为需要逐个元素的检查 String 中的值是否为空格，需要用 as_bytes 方法将 String 转化为字节数组
    println!("{:?}", s);
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    // 接下来，使用 iter 方法在字节数组上创建一个迭代器：
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // i是索引，item是元素
            return i;
        }
    }

    // 如果没有空格，则整个字符串就是首字母
    s.len()
}

// 获取首单词
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 形参也要注明引用
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn cancel_calculate() {
    /*
    ! 取消引用
    取消引用是指通过引用来访问值的过程。引用是一个指向值的地址，取消引用就是通过这个地址来访问值。
    */
    let x = 5;
    let r = &x; // r 是 x 的引用
    let y = *r; // *r 是取消引用，从引用中取出值
    println!("x = {x}, y = {y}");
}

fn info_reference() {
    // 取消引用
    cancel_calculate();

    // !使用 & 即可引用。
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 修改引用的值，则需要 &mut 将传入的参数转换为可变引用.
    change(&mut s1);
    println!("{s1}"); // hello, world
}

fn no_double_borrow() {
    // 不能同时拥有可变引用和不可变引用
    let mut s = String::from("hello");

    // 不可变引用 可以引用多次
    let r1 = &s; // 不可变引用
    let r2 = &s; // 不可变引用

    println!("{r1}");
    // 可变类型引用，只能引用一次
    let r3 = &mut s; // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{r3}");
    r3.push_str(", world");
    println!("{r3}");
}

pub fn demo() {
    // 引用借用是一个道理
    // info_reference();
    // 可变引用不能用两次
    // no_double_borrow();

    // 悬垂指针（dangling pointer）
    // let test = dangle(); // error[E0106]: missing lifetime specifier

    // 迭代器分割字符串
    // let mut data = String::from("ab c");
    // let mut res = first_word(&data);
    // data.clear(); // data会被清空
    // println!("{res}, {data}");

    // 改进
    // let mut data = String::from("ab c");
    // let res2 = first_word2(&data);
    // data.clear(); // 不能被清空，因为res2是一个引用data的部分index
    // println!("{res2}"); // 这里由于是引用分割，当你清空原始对象后，会报错。error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable

    // 字符串切割
    // let s = String::from("hello world");
    // let hello = &s[0..5];
    // let hello = &s[..5];
    // let endIndex = s.len();
    // let world = &s[6..11];
    // let world = &s[6..endIndex];

    // 获取整个字符串
    // let slice = &s[0..len];
    // let slice = &s[..];

    // println!("{s}, {hello}, {world}")
    // let x = &s[-1]; // 不支持负索引

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    // 读写
    for s in slice {
        println!("{}", s);
    }
    // 断言相等
    assert_eq!(slice, &[2, 3]);

    // 其他测试
    // split_strs();
}

fn split_strs() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word2(&my_string[0..6]);
    let word = first_word2(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word2(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word2(&my_string_literal[0..6]);
    let word = first_word2(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word2(my_string_literal);
}
