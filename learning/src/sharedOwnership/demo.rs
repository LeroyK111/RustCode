/*
!Rust标准库中内置了两种提供底层数据共享所有权的类型：Rc和Arc(分别是“引用计数”和“原子引用计数”)。

这两种类型都通过跟踪引用的数量并确保只要存在任何活动引用，数据就会持续存在，从而为所包含的数据提供共享所有权。

它们都实现了Clone和Drop：clone增加引用计数，drop减少引用计数。只要存在引用，数据就保持活跃，并且只有在所有clone超出范围时才会清理数据。

这两种类型都会产生少量的运行时开销来维护引用计数。它们之间的关键区别是Arc是线程安全的，而Rc不是。

Arc使用原子操作来管理引用计数，这给它带来了更高的运行时成本，但使它在线程之间的共享是安全的。如果你只在一个线程中工作，那么Rc是更快的选择。

关于Rc和Arc的最后一个重要事实是，它们只允许获得对底层数据的不可变引用。因为它们从根本上表示共享数据，允许可变引用会因为数据竞争和use-after-free错误而违反Rust的安全保证。


?打破唯一借用

如果我想在应用程序的各个部分之间共享可变数据怎么办？我们要打破的下一个借用检查器规则是唯一借用：为了改变某些东西，需要对数据有一个唯一的(也称为可变的)引用。借用检查器强制只能有一个可变引用或任意数量的不可变引用，但永远不能将两者组合在一起。

这种限制在大多数情况下都很有效，但我们要打破规则！在两种情况下，可能希望在没有可变引用的情况下改变数据：

缓存不可变计算(即缓存中间结果)


Rc或Arc共享所有权的情况下修改数据


为了打破编译时借用规则，编译器提供了几个有用的类型，每个类型都有自己的缺点。它们都可以让你安全地改变不可变引用后面的数据，使用不同的方法来确保你的程序仍然是安全的。
*/

#[derive(Debug)]
struct Pet {
    name: String,
}

impl Pet {
    fn new(name: String) -> Self {
        Self { name }
    }
}

struct Person {
    pets: Vec<Rc<Pet>>,
}

fn main() {
    // 创建两只共享所有权的宠物
    let cat = Rc::new(Pet::new("Tigger".into()));
    let dog = Rc::new(Pet::new("Chase".into()));

    // 创建一个同时拥有两只宠物的人
    let brother = Person {
        pets: vec![cat.clone(), dog.clone()],
    };

    // 创建另一个同时拥有两只宠物的人
    let sister = Person {
        pets: vec![cat, dog],
    };

    // 即使一方放弃所有权，另一方仍然拥有共同所有权
    drop(sister);
    println!("Pets: {:?}", brother.pets)
}

/*
!RefCell

首先是RefCell，它将唯一借用的强制执行从编译时移到了运行时。与Rc类似，RefCell使用引用计数来跟踪程序运行时有多少借用是活动的。

如果试图同时获取一个可变引用和另一个引用，RefCell将立即Panic！所以当使用RefCell时，你要确保你的程序不会同时读取和写入同一数据。

RefCell中的引用计数不是线程安全的，因此在线程之间共享RefCell数据是不可能的。为了在线程之间改变共享数据，我们需要使用下一个工具。
*/

struct FibonacciCalculator {
    cache: RefCell<HashMap<usize, usize>>,
}

impl FibonacciCalculator {
    // 计算第n个斐波那契数，缓存结果以防止重新计算
    // 注意，这需要' &self '，而不是' &mut self ' !
    fn calculate(&self, n: usize) -> usize {
        if n <= 2 {
            return 1;
        }

        // 检查缓存
        if let Some(value) = self.cache.borrow().get(&n) {
            return *value;
        }

        // 计算并缓存该值
        let result = self.calculate(n - 1) + self.calculate(n - 2);

        self.cache.borrow_mut().insert(n, result);

        result
    }
}


/*
!锁类型

接下来的两种类型是Mutex和RwLock，它们都提供了以线程安全的方式从不可变引用访问可变引用的方法。它们通过完全阻塞线程来实现这一点，直到可以安全访问数据为止。

这为访问的安全性提供了强有力的保证，但也有一个主要的缺陷：死锁。当两个线程在互相等待另一个线程释放持有的数据时，就会发生死锁。

类似于RefCell，它是由你来确保你的程序的逻辑不会发生死锁。

例如，下面的代码片段使用互斥锁在两个新线程中并行地增加计数器，然后从原始线程读取最终结果：

这些锁类型的线程安全特性使它们能够共享可变数据，但是它会带来巨大的性能成本：锁定线程，在数据可用之前无法执行其他工作。如果我们的数据足够简单，有一种类型可以在不需要锁的情况下跨线程提供共享可变访问。


!原子类型(Atomics)

原子类型可用于整型和布尔原语，这些类型都提供了将数据作为单个操作进行修改或读取的方法，因此中间不会发生任何事情，也不存在数据竞争的可能性。

例如，如果想增加一个计数器，而不是读取值，增加一个值，然后写入值，可以使用fetch_add方法在单个块中完成所有这些操作。

Arc在内部就是使用原子计数器以线程安全的方式管理引用计数。
*/


fn test() {
    // 创建一个共享的可变计数器
    let counter = Arc::new(Mutex::new(0));

    // 生成一个增加计数器的线程
    let counter1 = counter.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            *counter1.lock().unwrap() += 1;
        }
    });

    // 生成另一个增加计数器的线程
    let counter2 = counter.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            *counter2.lock().unwrap() += 1;
        }
    });

    // 等待线程完成
    handle1.join().unwrap();
    handle2.join().unwrap();

    // “Value: 20”，因为每个线程将计数器增加10。
    println!("Value: {}", *counter.lock().unwrap());
}

/*
总结

所有这些类型的美妙之处在于，它们提供了一种打破借用检查器规则的方法，同时仍然保证了Rust的安全性。

如果希望在不复制的情况下共享所有权，则需要引用计数类型。

如果在Rust借用检查器严格的可变引用规则下寻找可变性，可以选择内部可变性。

如果需要共享可变状态，可以使用它们的组合：Rc<RefCell<T>>或Arc<Mutex<T>>分别是单线程和多线程下共享可变状态的组合。
*/