/*
Paste crate允许在编译时连接标识符，这在编写宏以使用宏变量和静态文字创建任意标识符时非常有用。在下面的例子中，定义了一个宏，这个宏为一个名为$name的类型创建了一个impl块，并为每个$字段创建了getter方法。
*/

macro_rules! make_a_struct_and_getters {
    ($name:ident { $($field:ident),* }) => {
        // ...

        // Build an impl block with getters. This expands to:
        //     impl S {
        //         pub fn get_a(&self) -> &str { &self.a }
        //         pub fn get_b(&self) -> &str { &self.b }
        //         pub fn get_c(&self) -> &str { &self.c }
        //     }
        paste! {
            impl $name {
                $(
                    pub fn [<get_ $field>](&self) -> &str {
                        &self.$field
                    }
                )*
            }
        }
    }
}

make_a_struct_and_getters!(S { a, b, c });

fn call_some_getters(s: &S) -> bool {
    s.get_a() == s.get_b() && s.get_c().is_empty()
}