
/*
Cargo 1.80现在包含了对cfg名称和值的检查，以捕获拼写错误和错误配置，从而提高了条件配置的可靠性。


*/

fn main() {
    println!("Hello, world!");

    #[cfg(feature = "crayon")]
    rayon::join(
        || println!("Hello, Thing One!"),
        || println!("Hello, Thing Two!"),
    );
}