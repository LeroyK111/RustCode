# 在rust中我们常见到self和Self

一个是小写另一个是大写，大写的常见于trait和impl中，而小写的则常见于方法的参数声明中，很多人把它俩容易弄混淆。


## Self

- Self是一个类型别名，通常用于trait和impl块中，表示当前的类型或类型的类型。
- 在Trait定义中，Self代表了实现该trait的类型。
- 在impl中，Self可以用来引用当前实现类型的路径。


在这个例子中，Self在Trait MyTrait表示实现了MyTrait的类型，而在impl块中，Self指的是MyTrait。
```rust
trait MyTrait {
    fn method(&self) -> Self;
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn method(&self) -> Self {
        MyStruct
    }
}
```


## self

- self在rust中是一个关键字，用于引用当前对象的实例。在方法定义中，self是第一个参数，代表当前对象的引用。
- self可以是&self(借用当前实例),&mut self(可变借用当前实例)，或者self(获取当前实例的所有权)。

在这个例子中:
- Point::new使用Self作为返回类型，表示返回一个新的Point实例。
- Point::translate 方法实用&mut self作为第一个参数，表示该方法可变的借用Point实例，并具有修改该实例的能力，就是可以修改结构体Point中字段的值。

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn main() {
    let mut point = Point::new(1, 1);
    point.translate(2, 2);
    println!("Point coordinates: ({}, {})", point.x, point.y);
}
```

- Self是一个类型，用于引用当前的类型，而self是一个值，用于引用当前实例。
- Self通常在trait和impl块中使用，而self只能在方法定义中使用。
- Self可以用于返回类型，类型参数等。而self用于方法参数列表中，表示当前实例的引用。