// ! rust所有权，你需要知道的10条规则

fn main() {
    /*
    这里，x是整数值5的所有者。变量x跟踪并管理5的值。

    这种所有权系统避免了多个变量指向相同值的混乱情况。对于单一所有权，很明显x是数据5的唯一所有者。
    */
    let x = 5;

    /*
    y超出了作用域，5被删除
    作用域指的是变量在哪个块中有效，在上面的例子中，y只存在于{}花括号中。一旦执行离开该块，y就消失了，值5也被删除了。

    这种自动释放数据的方式避免了内存泄漏。一旦所有者消失，Rust就会清理这个值。再也不用担心悬空指针或内存膨胀了！
    */
    {
        let y = 5; // y是5的所有者
    }

    /*
        在这个例子中，将所有权从z转移到x的成本很低。有些语言使用引用计数，其中多个变量可以指向一个值，但这有开销。

    对于单一所有权，Rust只更新一个内部所有者变量来将所有权从z移到x。没有代价高昂的计数器更新。
        */
    let z = String::from("5");
    let x = z; // z的所有权移到了x
               // z不再拥有5

    /*
        这里我们创建字符串“hello”并将其绑定到s1。然后我们把s1赋值给一个新的变量s2。

    这将所有权从s1转移到s2。S1不再拥有字符串！数据本身没有被复制，只是所有权移动了。

    这可以防止意外地制作昂贵的副本。要真正复制数据，必须明确的使用Rust的clone()方法。
        */
    let s1 = "hello".to_string(); // s1拥有“hello”
    let s2 = s1; // s1的所有权转移到了s2
                 // s1不能再使用“hello”了

    /*
        &操作符创建了一个引用r，该引用在这个作用域中从s借用了所有权。

    可以把r看作是暂时借用s所拥有的数据，s仍然保留对数据的完全所有权，r只允许读取"hello"字符串。
        */

    let s = "hello".to_string(); // s拥有“hello”

    let r = &s; // r不变地借用s
                // s仍然拥有“hello”

    println!("{}", r);

    /*
        let mut s = "hello".to_string();

    let r1 = &mut s; // r1是s的可变引用

    let r2 = &mut s; // error!
        */
    let mut s = "hello".to_string();

    let r1 = &mut s; // r1是s的可变引用

    let r2 = &mut s; // error!
                     /*
                     Rust通过强制执行引用不能比其所有者的生命周期更长这个规则来防止在内存释放后再次使用引用。
                     */
    {
        let r = String::from("hello");
        drop(r);
        let s = &r;
        println!("{}", s);
    }

    /*
        结构体将相关数据组织在一起，但是所有权规则仍然适用于它们的字段。

    我们可以将结构体的所有权传递给函数和线程，或者不变地借用它们。
        */
    struct User {
        name: String,
        age: u32,
    }

    let user1 = User {
        name: "John".to_string(),
        age: 27,
    };
    let user2 = user1; // 所有权转移到user2
                       // user1不能再使用

    let borrow = &user1; // 通过引用借用user1
                         // user1仍然拥有数据

    /*
        这里s1是在栈上分配的，但包含一个指向堆分配的String。即使它在堆上，所有权规则也同样适用。

    Rust防止重复释放或在free之后使用，即使在使用指针时也是如此，所有权系统保证堆分配安全。
        */

    let s1 = String::from("hello"); // 栈上的s1拥有堆数据

    let s2 = s1.clone(); // 堆数据复制到新位置
                         // s1和s2各自拥有独立的数据

    let r = &s1; // r不变地借用s1的堆数据
                 // s1仍然拥有堆数据

    /*
        这里s1是在栈上分配的，但包含一个指向堆分配的String。即使它在堆上，所有权规则也同样适用。

    Rust防止重复释放或在free之后使用，即使在使用指针时也是如此，所有权系统保证堆分配安全。
        */

    use std::thread;

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    /*
        我们使用move闭包将v的所有权移到派生线程中。这可以防止多个线程并发访问v。

    在Rust中，所有权系统使得并发安全且简单。不需要锁，因为编译器强制单一所有权。
        */
}
