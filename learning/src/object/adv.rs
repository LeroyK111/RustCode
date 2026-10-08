// 定义 gui 模块，用来组织 GUI 框架相关的代码。
mod gui {
    // 定义公开的 Draw trait，规定“可绘制类型”必须提供什么行为。
    pub trait Draw {
        // 要求实现者提供 draw 方法。
        // &self 表示借用当前实例，不取得所有权，也不直接修改实例。
        // 这里只声明接口，具体绘制行为由各个类型自己实现。
        fn draw(&self);
    } // Draw trait 定义结束。

    // 定义公开的 Screen 类型，用来保存并统一处理多个组件。
    pub struct Screen {
        // components 是公开字段。
        // Vec：保存多个元素。
        // Box：拥有并存放具体组件。
        // dyn Draw：隐藏组件的具体类型，只保留 Draw 接口。
        // 每个元素都是 Box<dyn Draw>，但可以分别指向不同组件类型。
        pub components: Vec<Box<dyn Draw>>,
    } // Screen 结构体定义结束。

    // 为 Screen 定义它自己的方法。
    impl Screen {
        // 定义公开的 run 方法。
        // &self 表示只借用 Screen，不消耗它，因此之后还可以再次调用 run。
        pub fn run(&self) {
            // 借用组件集合进行遍历，不把集合或组件移出 Screen。
            // 每次得到的 component 是 &Box<dyn Draw>。
            for component in &self.components {
                // 通过 trait object 调用 Draw::draw。
                // 指向 Button 时调用 Button 的实现；
                // 指向 SelectBox 时调用 SelectBox 的实现。
                // 方法调用会自动处理这里的引用和 Box 解引用。
                component.draw();
            } // 本轮遍历结束，继续处理下一个组件，直到全部处理完。
        } // run 方法结束。
    } // Screen 的方法定义结束。

    // 定义公开的 Button 类型。
    pub struct Button {
        // 保存按钮文字。
        // 字段也是公开的，所以模块外可以构造 Button { label: ... }。
        pub label: String,
    } // Button 结构体定义结束。

    // 明确声明：Button 实现 Draw trait。
    // 这不是继承字段，而是为 Button 提供 Draw 要求的行为。
    impl Draw for Button {
        // 给出 Button 对 draw 方法的具体实现。
        fn draw(&self) {
            // 读取这个按钮的 label 字段，并打印。
            // 这里用打印模拟绘制，没有实现真正的图形界面。
            println!("绘制按钮：{}", self.label);
        } // Button 的 draw 实现结束。
    } // Button 对 Draw 的实现结束。
} // gui 模块结束。

// 把三个公开名称导入当前作用域，后面可以省略 gui:: 前缀。
// Button、Screen 是具体类型；Draw 是 trait。
use gui::{Button, Draw, Screen};

// 在 gui 模块外定义一个新组件。
// gui 框架本身没有定义或引用这个 SelectBox 类型。
struct SelectBox {
    // 保存下拉框的选项，每个选项是一个 String。
    options: Vec<String>,
}

// 为自己定义的 SelectBox 实现 gui 提供的 Draw trait。
// 因而 SelectBox 也可以通过 dyn Draw 接口被 Screen 使用。
impl Draw for SelectBox {
    // 定义下拉框自己的绘制行为。
    fn draw(&self) {
        // {:?} 使用调试格式打印整个选项列表。
        println!("绘制下拉框：{:?}", self.options);
    } // SelectBox 的 draw 实现结束。
}

pub fn test() {
    // 创建一个 Screen 实例，并把它绑定到变量 screen。
    let screen = Screen {
        // 根据 Screen 字段的声明，这里的集合类型是 Vec<Box<dyn Draw>>。
        components: vec![
            // 先创建 Button，再用 Box 拥有它。
            // 因为目标元素类型是 Box<dyn Draw>，
            // 编译器将 Box<Button> 转换为 Box<dyn Draw>。
            Box::new(Button {
                // 构造按钮的文字内容。
                label: String::from("确定"),
            }), // 第一个组件构造结束，并交给集合。
            // 创建 SelectBox，再用 Box 拥有它。
            // 它也实现了 Draw，因此可以转换为 Box<dyn Draw>。
            Box::new(SelectBox {
                // 创建包含两个字符串的选项列表。
                options: vec![String::from("是"), String::from("否")],
            }), // 第二个组件构造结束，并交给集合。
        ], // 组件集合构造结束，赋给 components 字段。
    }; // Screen 构造结束，开始由 screen 变量拥有。

    // 借用 screen，调用 run。
    // run 按集合顺序分别调用按钮和下拉框的 draw 实现。
    screen.run();
}

/*
动态分发: 主要是接口参数类型 是 &dyn
静态分发: 入参就是泛型
*/

mod blog {

    pub struct Post {
        // 当前状态：草稿、待审核或已发布
        state: Option<Box<dyn State>>,

        // 正文始终保存在 Post 中
        content: String,
    }

    impl Post {
        pub fn new() -> Self {
            Self {
                // 这里定义了当前的状态 draft
                state: Some(Box::new(Draft)),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            // 把“是否允许展示正文”交给当前状态决定

            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            // 取出旧状态的所有权，再放回转换后的状态
            // task 就是解引用
            if let Some(state) = self.state.take() {
                self.state = Some(state.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.approve());
            }
        }
    }

    trait State {
        // 消费当前状态，返回下一状态
        fn request_review(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        // 默认不允许展示正文
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft;
    struct PendingReview;
    struct Published;

    // 如果state是draft的话
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            println!("这里进入到了新状态");
            Box::new(PendingReview)
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            // 草稿不能直接发布，返回原状态
            // 依然返回 draft
            self
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            // 已经在审核，不重复转换
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            // 这里就是进入到新状态
            println!("再次获取新状态");
            Box::new(Published)
        }
    }

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            // 只有已发布状态提供真实正文
            println!("当published后,这里会替换原有实例的context");
            &post.content
        }
    }
}

use blog::Post;

pub fn test2() {
    /*
    ! todo 这里改成枚举,会更好用, 很多状态机都是枚举量
    */
    // 这就是个状态机
    let mut post = Post::new();

    post.add_text("今天继续学习 Rust。");

    println!("草稿：{:?}", post.content());
    assert_eq!("", post.content());

    // 故意跳过审核：应该仍然是草稿
    post.approve();
    assert_eq!("", post.content());

    post.request_review();

    println!("待审核：{:?}", post.content());
    assert_eq!("", post.content());

    post.approve();

    println!("已发布：{:?}", post.content());
    assert_eq!("今天继续学习 Rust。", post.content());
}
