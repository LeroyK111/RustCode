// 引用变量还在，不代表被引用的数据还在。
// fn area() {
//     let r;

//     {
// 这就是局部作用域, 一旦出来 就死掉了
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {r}");
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    /*
        | 语法         | 意思                 |
    | ---------- | ------------------ |
    | <'a>       | 声明一个生命周期参数，名字叫 'a  |
    | x: &'a str | x 是在 'a 期间有效的字符串引用 |
    | y: &'a str | y 也必须在这段期间有效       |
    | -> &'a str | 返回的引用在这段期间有效       |
        */
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn test() {
    // 它表示：两份借用都要能够支持共同的有效期间。返回值可能来自其中任意一个，所以调用者只能在两者共同支持的期间内使用它。
    let result = longest("1", "2");
    println!("{result}");

    // 内部使用例子
    let a = String::from("abcdef");
    {
        let b = String::from("xyz");
        let result = longest(a.as_str(), b.as_str());

        println!("{result}"); // a、b 都还有效
    }

    // 外部使用例子
    // let a = String::from("abcdef");
    // let result;
    // {
    //     let b = String::from("xyz");
    // `b` does not live long enough 本质就是b无法活到 result 再次生效的时候
    //     result = longest(a.as_str(), b.as_str());
    // } // b 已经被释放
    // println!("{result}"); // 不允许

    let a = String::from("abcdef");
    let result: String;

    {
        let b = String::from("xyz");
        // b给了所有权
        result = longest(a.as_str(), b.as_str()).to_owned();

        // 另一种写法
        // result = longest(&a, &b).clone();
    }

    println!("{result}"); // 可以：result 拥有自己的数据

    // 以下都是'的扩写
    fn largestAdv<T: PartialOrd>(list: &[T]) -> &T {
        &list[0]
        // 一个输入，输出跟它；方法借 self，输出默认跟 self；来源有歧义，手写关系。kaisery.github
    }

    fn largestAdv2<'a, T: PartialOrd>(list: &'a [T]) -> &'a T {
        &list[0]
    }

    struct Dog {
        name: String,
    }

    impl Dog {
        fn name<'a>(&'a self) -> &'a str {
            self.name.as_str()
        }
    }

    let s = Dog {
        name: String::from("2222"),
    };

    // !结构体
    /*
        | 定义            | 数据归谁所有     | 是否需要标注字段的引用生命周期 |
    | ------------- | ---------- | --------------- |
    | name: String  | 结构体拥有字符串   | 不需要             |
    | name: &'a str | 结构体借用外部字符串 | 需要              |
        */
    struct DogView<'a> {
        name: &'a str,
    }
    let name = String::from("小黑");
    let dog = DogView {
        // 这就是区别了, 如果是借用才能实例化,则必须加入'
        // name: name.as_str().to_owned(),
        name: name.as_str(),
    };
    println!("{}", dog.name);

    // 注入结构体的方法
    impl<'a> DogView<'a> {
        fn name(&self) -> &str {
            self.name
        }
    }

    // !这里字符串字面量对应的数据可以在整个程序运行期间有效。并不是说 text 这个局部变量永远不销毁。更多还是推荐 有生命周期的实例
    let text: &'static str = "hello";
}
