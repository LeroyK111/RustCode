pub mod genericsTemp;
pub mod lifeTemp;
pub mod traitTemp;
pub mod traitTemp2;



/*
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    let number = identity(10_i32);
    let flag = identity(true);
}

你只写了一份 identity<T>，编译器却会根据实际使用的类型生成具体版本。这个过程叫单态化。

*/