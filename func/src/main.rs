/*
! 本章我们学习闭包和迭代器。
*/


// 枚举结构
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

// 结构体
struct Inventory {
    // vec是列表
    shirts: Vec<ShirtColor>,
}


// ! 给结构体增加方法
impl Inventory {
    // * 如果 Option<T> 是 Some 成员，则 unwrap_or_else 返回 Some 中的值。 如果 Option<T> 是 None 成员, 则 unwrap_or_else 调用闭包并返回闭包的返回值。
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // 传参报错处理 || 传参正常处理函数
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    // 创建两个可变属性
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        // 循环列表
        for color in &self.shirts {
            // 对每个颜色判断
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        // 判断大小
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // *用结构体创建常量
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    // * 任意数据结构
    let user_pref1 = Some(ShirtColor::Red);
    // *调用结构体方法
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    // !试试none
    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref2, giveaway2
    // );
}
