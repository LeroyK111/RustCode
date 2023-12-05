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

    /*
    todo: Rust的#[cfg()]属性是一个强大的工具，可以在编译时根据指定的条件有条件地编译代码。它可以对代码的哪些部分包含或排除进行细粒度控制，从而产生更高效和优化的程序！通过混合使用条件和组合，开发人员可以根据不同的平台、特性和编译配置定制他们的代码库。
    */

    // 目标操作系统
    #[cfg(target_os = "android")]
    fn connect_to_instant_apps() {
        println!("test!")
    }
    // 目标架构
    #[cfg(target_arch = "x86_64")]
    fn target_system_config() -> Option<SystemConfig> {
        println!("test!")
    }
    // 功能标志
    #[cfg(feature = "server")]
    fn render(nodes: Vec<Node>) -> impl View {
        println!("test!")
    }
    // 调试模式
    #[cfg(debug_assertions)]
    fn debug_payload(payload: &Payload) {
        // Code runs only in debug mode
        println!("test!")
    }
    // 测试
    #[cfg(test)]
    fn save(&self) {
        // Don't persist since this isn't production code
        println!("test!")
    }
    // !组合
    // ALL
    // 仅在为32位体系结构的类unix操作系统时编译
    #[cfg(all(unix, target_pointer_width = "32"))]
    fn on_32bit_unix() {
        println!("test!")
    }
    // any
    // 这将在针对Android或iOS时编译
    #[cfg(any(target_os = "android", target_os = "ios"))]
    fn on_mobile() {
        println!("test!")
    }
    // 当不是Android或iOS时将编译
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn not_on_mobile() {
        println!("test!")
    }
    // !表达式
    // 确定Windows的缓存目录
    #[cfg(target_os = "windows")]
    const fn default_cache_dir() -> &'static str {
        "C:/cache"
    }

    // 确定Linux的缓存目录
    #[cfg(target_os = "linux")]
    const fn default_cache_dir() -> &'static str {
        "/mnt/c/cache"
    }   
}


// 模块化
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        // ...
    }
}

// 代码块
#[cfg(not(vendor_testing))]
{
    trace!("Starting payment processing");
    if let Ok(details) = process_payment(&transaction) {
        db.store(&details);
    }
    trace!("Ending payment processing");
}
#[cfg(vendor_testing)]
{
    debug!("Starting mock payment processing");
    if let Ok(details) = mock_process_payment(&transaction) {
        debug!("Payment details", &details);
    }
    debug!("Ending mock payment processing");
}

// 枚举变量
#[non_exhaustive]
pub enum Tab {
    Dashboard,
    Settings,
    #[cfg(feature = "admin")]
    Admin,
}

// 向量元素
impl Tab {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Dashboard,
            Self::Settings,
            #[cfg(feature = "admin")]
            Self::Admin,
        ]
    }
}

// 结构体
#[cfg(debug_assertions)]
#[derive(Default)]
pub struct DevBuildState {
  dev_id: Mutex<Option<String>>,
}

// 导入语句
#[cfg(debug_assertions)]
use crate::state::DevBuildState;

// 宏操作
window.set_title(if cfg!(feature = "admin") && user.is_admin {
    "Admin User Settings"
} else {
    "User Settings"
});


// 属性扩展
#[cfg_attr(feature = "magic", sparkles, crackles)]
fn bewitched() {}

// 当启用' magic '特性标志时，上述内容将扩展为
#[sparkles]
#[crackles]
fn bewitched() {}

/*
!习惯用法：
如果在运行程序时提供了额外的--cfg选项：
rustc --cfg "foobar" main.rs
或者通过在构建脚本中指示Cargo：
println!("cargo:rustc-cfg=foobar");
然后，你可以通过以下方式访问它：
#[cfg(foobar)]
*/
