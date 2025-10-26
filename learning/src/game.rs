// 当前子模块参数，对象，方法等全暴露给其他模块
// pub(crate)

// 标准库std，引入其中的io模块
use std::io;
// 引入三方随机数库
use rand::{self, Rng};
// 引入比较库
use std::cmp::Ordering;

// 只暴露这个函数
pub fn test() {
    println!("find me！");
    // 创建变量
    let apples: i32 = 5; // 不可变
    let mut bananas: i32 = 5; // 可变
                              // 模版字符串
    let x = 5;
    let y = 10;
    // 模板字符换
    println!("x = {x} and y + 2 = {}", y + 2);
}

pub fn g() {
    println!("Guess the number!");

    /*
    接下来，我们在中间还新增加了两行。
    第一行调用了 rand::thread_rng 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。
    接着调用随机数生成器的 gen_range 方法。这个方法由 use rand::Rng 语句引入到作用域的 Rng trait 定义。
    gen_range 方法获取一个范围表达式（range expression）作为参数，并生成一个在此范围之间的随机数。这里使用的这类范围表达式使用了 start..=end 这样的形式，也就是说包含了上下端点，所以需要指定 1..=100 来请求一个 1 和 100 之间的数。
    */

    // !随机数生成，再控制个范围，链式调用，太熟悉了
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    // 再套入循环
    loop {
        println!("Please input your guess.");
        // 创建一个 变量（variable）来储存用户输入，创建一个new string的空字符串实例
        let mut guess = String::new();

        // 写法一，如果你没有引用io，则标准库可以在这里引用
        // std::io::stdin()
        //     .read_line(&mut guess)
        //     .expect("Failed to read line");

        // 写法二
        io::stdin()
            // 我们还将 &mut guess 作为参数传递给 read_line() 函数，让其将用户输入储存到这个字符串中。并且read_line只能追加，不能覆盖
            .read_line(&mut guess) // 调用read_line方法从标准输入
            .expect("Failed to read line");
        
        





        /*
        todo 这里就是所有权了，也是rust的核心
        * & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数* * 据，而无需在内存中多次拷贝。引用是一个复杂的特性，Rust 的一个主要优势* 就是安全而简单的操纵引用。完成当前程序并不需要了解如此多细节。现在，我* 们只需知道它像变量一样，默认是不可变的。因此，需要写成 &mut guess 来* 使其可变，而不是 &guess。
        */

        /*
        todo expect捕捉异常
        read_line 会将用户输入附加到传递给它的字符串中，不过它也会返回一个类型为 Result 的值。 Result 是一种枚举类型，通常也写作 enum。枚举类型变量的值可以是多种可能状态中的一个。我们把每种可能的状态称为一种 枚举成员（variant）。
        */

        /*
        Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值。Err 成员则意味着操作失败，并且包含失败的前因后果。

        这些 Result 类型的作用是编码错误处理信息。Result 类型的值，像其他类型一样，拥有定义于其上的方法。Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息。如果 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。如果 Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。在本例中，这个值是用户输入到标准输入中的字节数。

        如果不调用 expect，程序也能编译，不过会出现一个警告：


        $ cargo build
        Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
        warning: unused `Result` that must be used
        --> src/main.rs:10:5
        |
        10 |     io::stdin().read_line(&mut guess);
        |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        |
        = note: `#[warn(unused_must_use)]` on by default
        = note: this `Result` may be an `Err` variant, which should be handled

        warning: `guessing_game` (bin "guessing_game") generated 1 warning
            Finished dev [unoptimized + debuginfo] target(s) in 0.59s
        Rust 警告我们没有使用 read_line 的返回值 Result，说明有一个可能的错误没有处理。

        消除警告的正确做法是实际去编写错误处理代码，不过由于我们就是希望程序在出现问题时立即崩溃，所以直接使用 expect。
        */

        // 模版字符串语法
        // println!("You guessed: {guess}");

        // 类型转换语法
        /*
        创建了一个叫做 guess 的变量。不过等等，不是已经有了一个叫做 guess 的变量了吗？确实如此，不过 Rust 允许用一个新值来 隐藏 （Shadowing） guess 之前的值。这个功能常用在需要转换值类型之类的场景。

        todo String 实例的 trim 方法会去除字符串开头和结尾的空白字符，我们必须执行此方法才能将字符串与 u32 比较，因为 u32 只能包含数值型数据。

       todo 字符串的 parse 方法 将字符串转换成其他类型。这里用它来把字符串转换为数值。我们需要告诉 Rust 具体的数字类型，这里通过 let guess: u32 指定。guess 后面的冒号（:）告诉 Rust 我们指定了变量的类型。Rust 有一些内建的数字类型；u32 是一个无符号的 32 位整型。
        */

        // * 语法糖，分别处理成功or失败,类型转换
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("please input int will be 1~100 ");
                continue},
        };

        // 直接补充缺失 范围
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        // 逻辑解耦
        let guess = Guess::new(guess.try_into().unwrap());
        if guess.braeking {
            continue;
        }

        // 开始比较, 但是必须类型一致，也就说我们必须转换字符串到u32
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                // 这个箭头函数真的强
                println!("you win");
                break;
            }
        }
    }
}

pub struct Guess {
    value: i32,
    braeking: bool,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        } else {
            Guess {
                value,
                braeking: false,
            }
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
