/*
迭代器和闭包
*/

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&mut self, user_preference: Option<ShirtColor>) -> Option<ShirtColor> {
        /*
            todo || 这里就是闭包的关键
            || 没有需要传递的err
            |erros| 有需要传递的err
            self.most_stocked() 执行的函数
            self 获取外部对象

            传入计算结果：调用 unwrap_or 前就计算好了
            user_preference.unwrap_or(self.most_stocked())

            传入计算办法：只有需要时才计算
            user_preference.unwrap_or_else(|| self.most_stocked())
            闭包在这里的价值就是延迟执行，但不是后台执行，也不是创建线程。



        */

        // 再来理解 unwrap or else
        // 第一步：定义备用办法，暂时不执行
        // let chooseDefault = || self.most_stocked();
        // 第二步：交给 unwrap_or_else，在需要时调用
        // user_preference.unwrap_or_else(chooseDefault)

        let color = user_preference.unwrap_or_else(|| self.most_stocked());

        // 找到坐标
        let position = self.shirts.iter().position(|x| *x == color);

        match position {
            Some(index) => {
                let removed = self.shirts.remove(index);
                Some(removed)
            }
            None => {
                // 没有该颜色，或者库存为空
                None
            }
        }

        /*
        Option：没有值时执行，闭包没有参数
        option.unwrap_or_else(|| fallback())

        Result：有错误时执行，把错误传给闭包
        result.unwrap_or_else(|error| handle_error(error))
        || 与 |error| 不是随意选的，取决于调用者会提供什么参数。
                */
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        println!("红色{num_red}, 蓝色{num_blue}");
        if num_red >= num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn cloth() {
    // 构建一个类集合
    let mut store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    //
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    // println!("用户 {:?} 商店服务对象 {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("用户 {:?} 服务对象 {:?}", user_pref2, giveaway2);

    // 深入理解闭包
    let someOne = "haha";
    let add = |a: i32, b: i32| -> i32 {
        println!("正在执行加法{}", someOne);
        a + b
    };
    fn add2(a: i32, b: i32) -> i32 {
        // 闭包就可以取到外层实体, 但是函数不行, 必须入参
        // println!("正在执行加法{}", someOne);
        a + b
    }

    let result = add(2, 3);
}

fn example1() {
    let values = vec![10, 20, 30];
    let mut iter = values.iter();

    println!("{:?}", iter.next()); // Some(&10)
    println!("{:?}", iter.next()); // Some(&20)
    println!("{:?}", iter.next()); // Some(&30)
    println!("{:?}", iter.next()); // None
    println!("{:?}", iter);
    println!("{:?}", values);

    for value in values.iter() {
        println!("{value}");
    }
}

fn example2() {
    // 借用
    let values = vec![1, 2, 3];
    for value in values.iter() {
        println!("{value}");
    }
    // values 仍然存在
    println!("{values:?}");

    // 改用
    let mut values = vec![1, 2, 3];

    for value in values.iter_mut() {
        *value += 10;
    }

    println!("{values:?}"); // [11, 12, 13]

    // 彻底拿走
    let values = vec![String::from("红色"), String::from("蓝色")];

    for value in values.into_iter() {
        // value 是 String，而不是 &String
        println!("{value}");
    }
}

fn example3() {
    let values = vec![1, 2, 3];

    let mapped = values.iter().map(|x| {
        println!("正在处理：{x}");
        *x + 1
    });

    // 到这里，上面的 println! 一次都没有执行
    let result: Vec<i32> = mapped.collect();

    // collect 开始取元素，闭包才逐个执行
    println!("{result:?}"); // [2, 3, 4]

    let values = vec![1, 2, 3];
    let iter = values.iter();

    let total: i32 = iter.sum();

    println!("{total}"); // 6
    println!("{values:?}"); // [1, 2, 3]

    // todo map
    let values = vec![1, 2, 3];
    // collect 再次转成集合
    let result: Vec<i32> = values.iter().map(|x| *x + 1).collect();

    // result：[2, 3, 4]
    // values：[1, 2, 3]，原集合没变

    // todo 筛选
    let values = vec![1, 2, 3, 4, 5];

    let minimum = 3;

    let result: Vec<i32> = values.into_iter().filter(|x| *x >= minimum).collect();

    // result：[3, 4, 5]

    // !最大例子
    let values = vec![1, 2, 3, 4, 5];

    let result: Vec<i32> = values
        .into_iter() // 来源：逐个取出数字
        .filter(|x| *x >= 3) // 规则：保留大于等于 3 的
        .map(|x| x * 10) // 规则：把留下的数字乘以 10
        .collect(); // 执行：收集到新 Vec

    assert_eq!(result, vec![30, 40, 50]);
}

pub fn nextItem() {
    // example1();
    // example2();
    example3();
}
