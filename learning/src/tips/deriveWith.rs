/*
通过派生宏 #[derive(With)] 给结构体字段生成 with_xxx 方法，通过链式调用 with_xxx 方法来构造结构体。
*/

#[derive(With)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}

impl Foo {
    pub fn with_a(mut self, a: impl Into<i32>) -> Self {
        self.a = a.into();
        self
    }
    pub fn with_b(mut self, b: impl Into<String>) -> Self {
        self.b = b.into();
        self
    }
}

#[derive(With)]
pub struct Bar(i32, String);

impl Bar {
    pub fn with_0(mut self, field_0: impl Into<i32>) -> Self {
        self.0 = field_0.into();
        self
    }
    pub fn with_1(mut self, field_1: impl Into<String>) -> Self {
        self.1 = field_1.into();
        self
    }
}

#[derive(With)]
#[with(a)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}

impl Foo {
    pub fn with_a(mut self, a: impl Into<i32>) -> Self {
        self.a = a.into();
        self
    }
}

#[derive(With)]
#[with(1)]
pub struct Bar(i32, String);

impl Bar {
    pub fn with_1(mut self, field_1: impl Into<String>) -> Self {
        self.1 = field_1.into();
        self
    }
}
