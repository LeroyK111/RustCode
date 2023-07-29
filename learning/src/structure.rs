// 定义结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// 元组结构体
struct Color(i32, i32, i32);

// 类单元结构体
struct AlwaysEqual;

pub fn theWorld() {
    // 有点对象语法的感觉
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // 结构体赋值
    user1.email = String::from("785632486@qq.com");
    // println!("{}", user1.email);

    // 使用user1的内容，填充给user2
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // 使用语法糖，进行解构赋值
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user1被清理掉了，user3能正常使用。
    // println!("{}", user1.email)
    // println!("{}", user2.email)
    // println!("{}", user3.sign_in_count);
    // clone 可以进行属性的深复制, 但结构体不能深复制
    let user4 = user3.email.clone();
    // println!("{}", user4)

    // 元组结构体
    let black = Color(0, 0, 0);

    // 类单元结构体
    let subject = AlwaysEqual;

    // 结构体传参数
    // wh();

    // 结构体原型，方法
    // print();

    // 结构体参数增加
    // inherit();

    // !关联函数, 涉及到模块语法
    testSelf();
}

// 使用 #[derive(Debug)] 后才能打印结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 计算矩形面积
fn wh() {
    // let width1 = 30;
    // let height1 = 50;
    // println!("{}", area(width1, height1));

    // let rect1 = (30, 50);
    // println!("{}", area1(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area2(&rect2));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    // 使用元组结构体，计算
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    // 使用普通对象结构体
    rectangle.height * rectangle.width
}

// 改写结构体，这里很像js的prototype，高级对象方法。
impl Rectangle {
    fn area(&self) -> u32 {
        // 这个self又类似
        self.width * self.height
    }
    // 调用内部方法的返回值，类似计算属性的引用，
    fn width(&self) -> bool {
        self.width > 0
    }
}

// 通过派生 trait 增加实用功能, 打印结构体
fn print() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // 非格式化语法
    // println!("rect1 is {:?}", rect1);
    // 格式化语法
    // println!("rect1 is {:#?}", rect1);

    // 另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏。dbg! 宏接收一个表达式的所有权（与 println! 宏相反，后者接收的是引用），打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。

    // 打印到异常控制台
    // dbg!(&rect1);

    // 调用结构体内部方法
    println!("{}", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

impl Rectangle {
    // 添加新的结构体方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 结构体参数修改
fn inherit() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// 结构体的关联函数
impl Rectangle {
    // 这里就涉及到模块了
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn testSelf() {
    let sq = Rectangle::square(23);
    println!("{:#?}", sq)
}
