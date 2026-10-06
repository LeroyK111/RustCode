fn main() {
    // println!("Hello, world!");

    let num = 10;
    // 这里就是跨包运行
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
