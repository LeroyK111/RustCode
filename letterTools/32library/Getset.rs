// 以前的Java程序员会喜欢这个crate。Getset crate包含用于生成getter和setter方法的过程性宏。
use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Getters, Setters, MutGetters, CopyGetters, Default)]
pub struct Foo<T>
where
    T: Copy + Clone + Default,
{
    #[getset(get, set, get_mut)]
    private: T,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    public: T,
}

fn main() {
    let mut foo = Foo::default();
    foo.set_private(1);
    (*foo.private_mut()) += 1;
    assert_eq!(*foo.private(), 2);
}