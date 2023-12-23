/*
! 模板方法
Rust提供了trait，类似于面向对象的接口，不同的是，将传统面向对象的虚函数表从对象中分离出来，trait仍然是一个函数表，只不过是独立的，它的参数self指针可以指向任何实现了该trait的结构。
从对象中分离出虚函数表的trait，带来了使用上与面向对象一些根本的不同，这在我看来算是“很大”的不同了。让我们以模版方法设计模式为例来感受一下。先想一下，rust怎么依赖trait和结构继承，实现模板方法？所谓模板方法，就是父类留一个空白方法作为虚函数，交给子类实现，这样子类只负责不同的算法部分，是面向对象中很基础很常用的手法了。用Rust语言照葫芦画瓢先描述一下大概框架，如下：
/// 一个父类A
struct A {
    ...
}

impl A {
    fn do_step1_common(&mut self) { ... }

    // 缺省实现，留给子类实现，如果是C++/Java这类面向对象语言，很容易。若是Rust，该怎么搞？
    fn do_step2_maybe_different(&mut self) { ... }

    fn do_step3_common(&mut self) { ... }

    pub fn do_all_steps(&mut self) {
        self.do_step1_common();
        self.do_step2_maybe_different();
        self.do_step3_common();
    }
}

// 具体的某个子类实现
struct A1 {
    a: A,
    ...
}

impl A1 {
    // 开始实现
    fn do_step2_maybe_different(&mut self) {
        // A1提供一种实现
    }
}
不瞒大家，我初识rust时就被这样一个面向对象上的简单案例，用rust实现给难住了！当时卡在父类看起来像是一个完整的类型，Rust怎么能未卜先知调用子类的方法呢？
其实，Rust要想实现这种效果，不能A1继承A这种了，而是A包含A1子类来实现，反着来，将不同的实现单独拆出来作为trait，再交给子类实现。
trait DoStep2 {
    fn do_step2_maybe_different(&mut self);
}

/// 另一个父类B
struct B<T: DoStep2> {
    t: T, // 或者Box<&dyn DoStep2>
    ...
}

impl<T> B<T> {
    fn do_step1_common(&mut self) { ... }

    fn do_step3_common(&mut self) { ... }
}

impl<T: DoStep2> B<T> {
    pub fn do_all_steps(&mut self) {
        self.do_step1_common();
        self.t.do_step2_maybe_different();
        self.do_step3_common();
    }
}

/// 具体的子类实现
struct B1 {
    ...
}

impl DoStep2 for B1 {
    fn do_step2_maybe_different(&mut self) {
        // B1提供一种实现
    }
}

// 这样，
// B<B1> 相当于面向对象中的 A1
// B<B2> 相当于面向对象中的 A2
感觉不错，看起来颇为妥当，这种方式已经能在适合它的场景中工作，也是模板方法的体现。对比下，A、B都不是完整的父类实现，A1、B<B1>才是真正的具体类型，且它们都包含了父类的结构，虽然B<B1>的写法有点不合常规。若子类还拥有自己的独立的扩展结构的话，那Rust这种方式更优雅一些，拆分的更原子、更合理。实践中，往往不会这么完美的套用，会复杂很多，比如子类作为具体类型，想访问父类的成员，才能配合完成do_step2，Rust又该怎么做？面向对象的this指针则轻松支持。Rust不可能让B1再直接包含B，那样循环包含了，只能用引用或者指针来存在B1里面，但这样的话，岂不是太麻烦了，循环引用/包含都是我们极力避免的东西，麻烦到都想放弃模板方法了！
为何会有这种差异？因为面向对象的子类this指针其实指向的是整体，子类的函数表是个本身就包含父类的整体；而上述为B1实现DoStep2 trait的时候，self指向的仅仅是B1，并不知道B的存在。那怎么办？得让self指向整体B<B1>，那为B<B1>实现DoStep2行不行？像下面这样：
impl DoStep2 for B<B1> {
    fn do_step2_maybe_different(&mut self) {
        // 这里self可以访问“父类”B的成员了
    }
}
但回过头来，B::do_all_steps(&mut self)就没法在“父类”B中统一实现了，因为B<T>在B<B1>具象化之前，还不知道哪来的do_step2，因此要在impl B<B1>中实现，每个不同的具像化的子类都得单独实现相同的do_all_steps!你能接受不？
也许你能接受，为每个B<B1>、B<B2>...重复拷贝一遍各自的do_all_steps！本文基于专业探讨，还是要寻找一下编写通用的do_all_steps方法的，有没有？当然是有的，前提是，你得把do_step1_common，do_step3_common也得trait化，然后在用一个trait组合限定搞定，如下：
trait DoStep1 {
    fn do_step1_common(&mut self);
}

trait DoStep3 {
    fn do_step2_common(&mut self);
}

// 因为B<T>是泛型，只需为泛型编码实现一次DoStep1、DoStep3就行
impl<T> DoStep1 for B<T> { ... }
impl<T> DoStep3 for B<T> { ... }


// 最后，实现通用的do_all_steps，还得靠泛型。
// 此时，B<B1>已经满足T，会为其实现下面的函数
// 可以这样读：为所有实现了DoStep1/DoStep2/DoStep3特质的类型T实现do_all_steps
impl<T> T 
where
    T: DoStep1 + DoStep2 + DoStep3
{
    pub fn do_all_steps(&mut self) {
        self.do_step1_common();
        self.do_step2_maybe_different();
        self.do_step3_common();
    }
}
如何，这样应该能接受了吧。Rust通过把问题解构的更细粒度，完成了任务。客观对比下，面向对象的实现还是简单些，父类的do_step1和do_step3函数永远指向了同一个实现，而Rust靠泛型应该是指向了3个不同的实现？不知道编译期有没有优化，盲猜应该有。可以说语法如此，Rust只能做到如此了。与面向对象的模板方法相比，最后一点小瑕疵，就是要多定义DoStep1、DoStep2 2个trait，并用一个T: DoStep1 + DoStep2 + DoStep3通用类型包含同样实现了DoStep1 + DoStep2 + DoStep3的B<T>，进而代表它。可我们想仅仅为B<T>类型实现，其他类型也不太可能这样实现了，一个T则把范围不必要地扩大了。要是能按照我们想要的，就仅为B<T>且实现了DoStep2的B<T>来实现do_all_steps，就完美了。要做到此种程度，必须能对自身Self进行限定，如下：
/// 可以这样读：为所有自身实现了DoStep2的B<T>实现do_all_steps
impl<T> B<T>
where
    Self: DoStep2
{
    pub fn do_all_steps(&mut self) {
        self.do_step1_common();
        self.do_step2_maybe_different();
        self.do_step3_common();
    }
}
这种写法还真可以，也不用额外定义DoStep1、DoStep3了，因为本身B<T>已经有do_step1_common/do_step3_common的实现了，Rust最新的稳定版就支持这样写！
一段完整的Rust代码，可以参考这里：https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b80de6d4e6d75bf59bb37db386264fed
一个小小的模板方法，Rust分离出2种不同的方式，这是模板方法设计模式都没提到的，2种方式还各有韵味。从定义的顺序上，C++的模板方法，是 “子类后续扩展父类” ，Rust的模板方法，则是 “父类提前包含子类泛型” ，写法上还真是一开始不太好扭过来。可一旦扭转过来，发现Rust挺强，仍不失面向对象技巧。
反观面向对象，一个模板方法案例，让大家看到了些许面向对象的束缚，其实也无伤大雅，面向对象也能用纯组合的方式实现模板方法，也不用继承，如果需要组合的对象再通过构造动态传递进来，那就跟策略模式很像了，这种组合传递来的对象不止一个时，就是策略模式！然后，让我想起了一个小争论，子类应该严格不准访问父类的成员，让父类的变化完全掌控在父类手中。面向对象的确可以做到，全部private。但Rust的处理方式，显示出了其对这些细节的语法表达更合乎逻辑。
总结
模板方法是面向对象虚函数继承的基本应用，是面向对象很多设计模式的基础，如装饰器模式。一篇讲解下来，Rust从一开始别别扭扭到更好地支持模板方法，其实能体会到，Rust强迫你去拆解，即便都是同一个模板方法，但不同的细节要求，子类是否需要访问父类，都有不同的处理变化，分出来的形式还更严格。写到最后，Rust都感觉不到面向对象那味了，那是什么味？

*/

