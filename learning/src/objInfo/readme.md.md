# Rust面向对象

跟着大佬学习一下。
## 面向对象之结构继承

```
一切先从最简单的开始。面向对象届有句老话：“多用组合，少用继承”，被奉为圭臬，很多对代码的批评就源自于没遵守它。让我们对比一下：

struct A {  
    x: int,  
}  
  
struct B1 extends A {  
    y: int,  
}  
  
struct B2 {  
    a: A,  
    y: int,  
}  
  

B1好，还是B2好？其实无法回答的。然而，看下B1、B2这两者的内存布局，会惊讶的发现，它们其实是一样的。

看到这里，我第一次怀疑，这么多年的“继承” vs “组合”的争论，是否有必要，我们怎么会这么傻，为了一个相同的东西，还能喋喋不休了这么多年🐶。

不过，有人可能会问了，这种只是内存布局一样，实际上能一样吗，B1、B2对x的访问是不一样的。

// B1是这样访问x的  
int get_x = b1.x;  
  
// 而B2是通过a访问的  
int get_x = b2.a.x;  

确实不一样，但实际上`b2.x`可以看成`b2.a.x`的语法糖。到最终编译到的汇编语言层面，真也的确如此。Rust语言可以轻松使用Deref特质实现该语法糖，连调用A的方法都能一并简化。

impl Deref for B2 {  
    type Target = A;  
  
    fn deref(&self) -> &Self::Target {  
        &self.a  
    }  
}  

所以，别在讨论，该用组合还是该用继承了！继承本身就是组合，还有啥好讨论的，就算组合多个不同结构，也等同于多继承的概念。

了解到这一点，对底层而言，"is-a"就是"has-a"，高级语言为其发明了“继承”，此时显的多此一举。底层结构上都一样，所以go语言的结构继承，看上去就是组合，rust亦如此。继承就是一种特殊的组合！

但组合可不仅仅是继承，组合变化更多。组合还可以包含一个“指针”型结构，"has-a-ref"，这在链表、树这种自包含结构里尤其关键；组合还能包含一个集合——"has-many"，"has-a"都可以看成是"has many"的特例，比如树包含不止一个枝干。

// 表达能力上，B3更强，B1、B2都可以用B3来表示，B2不过是B3中包含长度为1的向量  
struct B3 {  
    a: Vec<A>,  
    y: int,  
}  

面向对象的一个尴尬就是，本身继承的底层结构是组合，功能上也不如组合，却把“继承”提高到“三大概念”的核心层次，因小失大，以偏概全。从这点看，"has-a"拥有相同结构，加上"is-a"的语法糖，所以go、rust，概念更少，还能表达出“继承”，亦不失组合的含义，更受欢迎。

但话说回来，面向对象，其实有个关键的语法习惯改进，即以对象为主语的调用方式，类自然语言的“主-谓”或者“主-谓-宾”语句，终于不用主语倒置了，现在大部分语言都是默认如此了，Rust也是如此。这时候再考虑继承，若论直接使用父类的谓词行为，则是“is-a”的继承独有的，“has-a”/“has-a-ref”都要借助语法糖或者重新实现父类接口来表达这种"is-a"父类的行为。“has-many”如果也能表现出"is-a"的特性，那就是经典的组合模式了。不过大多时候是“has-many”表现不出“is-a”的特性，仅仅是一种集合管理。

impl DrawWidget for A {  
    fn draw(&self, canvas: &mut Canvas) {  
        ...  
    }  
}  
  
// B1已天然实现了DrawWidget，仍可选覆盖实现  
// impl DrawWidget for B1 { ... }  
  
// B2则需要实现“is-a”。在Rust语言里，即便B2实现了Deref，也不代表着  
impl DrawWidget for B2 {  
    fn draw(&self, canvas: &mut Canvas) {  
        self.a.draw(canvas);  
        // draw y  
    }  
}  
  
// B3是“has-many”，但本身也可以看成是一个Widget的话，那就是面向对象中经典的组合模式  
impl DrawWidget for B3 {  
    fn draw(&self, canvas: &mut Canvas) {  
        for child in self.a {  
            child.draw(canvas);  
        }  
        // draw y  
    }  
}  

总结一下，"has"包括

- "has-a"
    

- 若意义等同于"is-a"，则为继承；包括"has-(a,b)"等同于"is-(a,b)"型，多继承概念
    
- 若不等同于"is-a"，则为简单的包含关系，如一个数据包包含包头和包体，一个分数包含分子和分母等
    
- 有时候，即可以用继承，也可以用组合，比如代理模式，就有继承代理和组合代理2种，其结构本相同，何须再分出你我，这就是面向对象不必要的高级概念复杂化
    
- "has-a"还可以是"has-a-ref"，C/C++中包含一个其他结构的指针成员，内存结构不同于继承，却也能形如继承，是组合的一种，链表这类自包含结构必备
    

- "has-many"
    

- 若还有跟"is-a"一样的特性，就是组合模式
    
- 普通的集合管理
    

回到面向对象语义/语言，要问该选哪一种，就看那种能更精准地表达，猫是动物、鸭子是动物，这种就是继承，猫、鸭子继承了动物，肯定比猫、鸭子包含一个动物结构好。而树包括枝干，就是包含关系好。高级语言，对同一结构不同表达，怎么方便人理解怎么来，如此而已。

在Rust语言中，则没了继承的概念，都是组合。因为Rust的Deref，让Rust保留了继承的部分功能性，并没有关闭面向对象的大门。但需注意，B2并未因Deref自动继承实现A所有的特质。Rust舍弃了高级语言复杂的“继承”概念，把底层是什么样就是什么样的组合原汁原味地展现出来，同时保留下其他变化，既继承的弊端可以被摒弃替换成组合或者特质实现的变化，这种变化也许才是那更常见的大多数情况，废除“继承”可能会是未来语言的标准做法。

## 结束语

本篇到这里暂时就结束了，因为篇幅不宜太长，先行文至此，一篇围绕一个小主题。本文仅仅讨论了面向对象的结构继承。面向对象的结构继承，其实狭隘了，从内存结构布局上看，仅仅是组合的一种特例，还不如说成就是组合，组合意义更广泛。有时候，不妨把底层的概念直接暴露出来，也没增加复杂度，理解上会更直白。

Rust也为结构继承留下了Deref的方案，不过请留意，Deref并没让子类自动继承实现父类的特型，只是一个解引用的语法糖，而且一个结构只能实现一次Deref到一个Target。Deref并非仅为结构继承而生，Rust也没怎么提倡用Deref式的继承，官方文档从来没说它是用来对标“继承”的，倒是不少开源项目，拿它映射继承，如果符合“is-a”的意义，还挺适用的。

继承仍有些问题、冲突没展示出来，后面再继续探讨。包括Deref这种具备关联类型的特质，它到后面也不再仅是一个语法糖，在表达实现逻辑语义时，子类没必要实现父类的特型，在特型限定时它们将有更丰富的逻辑表达意义。
```


## 模板方法

```
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

感觉不错，看起来颇为妥当，这种方式已经能在适合它的场景中工作，也是模板方法的体现。对比下，`A`、`B`都不是完整的父类实现，`A1`、`B<B1>`才是真正的具体类型，且它们都包含了父类的结构，虽然`B<B1>`的写法有点不合常规。若子类还拥有自己的独立的扩展结构的话，那Rust这种方式更优雅一些，拆分的更原子、更合理。实践中，往往不会这么完美的套用，会复杂很多，比如子类作为具体类型，想访问父类的成员，才能配合完成`do_step2`，Rust又该怎么做？面向对象的this指针则轻松支持。Rust不可能让`B1`再直接包含`B`，那样循环包含了，只能用引用或者指针来存在`B1`里面，但这样的话，岂不是太麻烦了，循环引用/包含都是我们极力避免的东西，麻烦到都想放弃模板方法了！

为何会有这种差异？因为面向对象的子类this指针其实指向的是整体，子类的函数表是个本身就包含父类的整体；而上述为`B1`实现DoStep2 trait的时候，self指向的仅仅是`B1`，并不知道`B`的存在。那怎么办？得让self指向整体`B<B1>`，那为`B<B1>`实现DoStep2行不行？像下面这样：

impl DoStep2 for B<B1> {  
    fn do_step2_maybe_different(&mut self) {  
        // 这里self可以访问“父类”B的成员了  
    }  
}  

但回过头来，`B::do_all_steps(&mut self)`就没法在“父类”B中统一实现了，因为`B<T>`在`B<B1>`具象化之前，还不知道哪来的`do_step2`，因此要在`impl B<B1>`中实现，每个不同的具像化的子类都得单独实现相同的`do_all_steps`!你能接受不？

也许你能接受，为每个`B<B1>`、`B<B2>`...重复拷贝一遍各自的`do_all_steps`！本文基于专业探讨，还是要寻找一下编写通用的`do_all_steps`方法的，有没有？当然是有的，前提是，你得把`do_step1_common`，`do_step3_common`也得trait化，然后在用一个trait组合限定搞定，如下：

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

如何，这样应该能接受了吧。Rust通过把问题解构的更细粒度，完成了任务。客观对比下，面向对象的实现还是简单些，父类的`do_step1`和`do_step3`函数永远指向了同一个实现，而Rust靠泛型应该是指向了3个不同的实现？不知道编译期有没有优化，盲猜应该有。可以说语法如此，Rust只能做到如此了。与面向对象的模板方法相比，最后一点小瑕疵，就是要多定义`DoStep1`、`DoStep2` 2个trait，并用一个`T: DoStep1 + DoStep2 + DoStep3`通用类型包含同样实现了`DoStep1 + DoStep2 + DoStep3`的`B<T>`，进而代表它。可我们想仅仅为`B<T>`类型实现，其他类型也不太可能这样实现了，一个T则把范围不必要地扩大了。要是能按照我们想要的，就仅为`B<T>`且实现了`DoStep2`的`B<T>`来实现`do_all_steps`，就完美了。要做到此种程度，必须能对自身Self进行限定，如下：

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

这种写法还真可以，也不用额外定义DoStep1、DoStep3了，因为本身`B<T>`已经有`do_step1_common`/`do_step3_common`的实现了，Rust最新的稳定版就支持这样写！

一段完整的Rust代码，可以参考这里：https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b80de6d4e6d75bf59bb37db386264fed

一个小小的模板方法，Rust分离出2种不同的方式，这是模板方法设计模式都没提到的，2种方式还各有韵味。从定义的顺序上，C++的模板方法，是 **“子类后续扩展父类”** ，Rust的模板方法，则是 **“父类提前包含子类泛型”** ，写法上还真是一开始不太好扭过来。可一旦扭转过来，发现Rust挺强，仍不失面向对象技巧。

反观面向对象，一个模板方法案例，让大家看到了些许面向对象的束缚，其实也无伤大雅，面向对象也能用纯组合的方式实现模板方法，也不用继承，如果需要组合的对象再通过构造动态传递进来，那就跟策略模式很像了，这种组合传递来的对象不止一个时，就是策略模式！然后，让我想起了一个小争论，子类应该严格不准访问父类的成员，让父类的变化完全掌控在父类手中。面向对象的确可以做到，全部private。但Rust的处理方式，显示出了其对这些细节的语法表达更合乎逻辑。

## 总结

模板方法是面向对象虚函数继承的基本应用，是面向对象很多设计模式的基础，如装饰器模式。一篇讲解下来，Rust从一开始别别扭扭到更好地支持模板方法，其实能体会到，Rust强迫你去拆解，即便都是同一个模板方法，但不同的细节要求，子类是否需要访问父类，都有不同的处理变化，分出来的形式还更严格。
```

## 策略模式

```
上节说到，模板方法变化一下就能成策略模式，怎么变化的？且看策略模式典型案例：

pub trait Fly {  
    fn fly(&self);  
}  
  
pub trait Quack {  
    fn quack($self);  
}  
  
/// 先以静多态的方式实现  
/// 似 trait Fly + Quack就是Duck，只是Fly和Quack独立地变化  
struct Duck<F, Q>   
where  
    F: Fly,  
    Q: Quack,  
{  
    fly_behabior: F,      // 单看这个成员，与模版方法如出一辙  
    quack_behavior: Q,    // 一样，将不同的算法部分交给子类去实现  
}  
  
impl<F, Q> Duck<F, Q>   
where  
    F: Fly,  
    Q: Quack,  
{  
    pub fn new(fly_behavior: F, quack_behavior: Q) {  
        Self { fly_behavior, quack_behavior }  
    }  
}

以上是策略模式的简单案例，策略模式可以说是模板方法的衍生变化。还记得上一章中第一种模板方法的实现方式不，单看Fly就是模板方法：模板方法里子类完全不依赖父类，干净地完成算法策略，那子类就能够依赖注入到父类中；最好这种子类不止一个，比如不仅有Fly还有Quack，就是纯正的策略组合模式了。了解这种变化可以帮助区分二者，比那说不清道不明的优缺点、适用场景描述能让你更清晰、透彻地认识到两者的差别与联系。

策略模式，公认的妙。上面是静多态实现的策略模式，会遇到类型爆炸的问题，比如有2种飞行方式、3种呱呱叫方式，那总共有2*3=6种复合类型，体现了组合是类型系统中的积类型。在嵌入式上，因为内存环境限制，类型爆炸导致程序大小变大成了问题，不得不改用动多态，以减少类爆炸带来的影响。

/// 动多态，类型统一了，类型也不会爆炸了  
struct DynamicDuck {  
    fly_behavior: Box<dyn Fly>,  
    quack_behavior: Box<dyn Quack>,  
}  

面向对象语言，都是动多态，Java对象皆引用，当引用没地方用了就垃圾回收；C++没有指针则玩不转面向对象，只可能将子类指针赋值给父类指针来多态，无法将子类对象赋值给父类对象来多态吧！所以面向对象的策略模式是动多态，天然无类型爆炸问题。

那类型爆炸一定差吗，类型统一就肯定好吗？先讨论下类型爆炸合理不。自然界生物划分“界门纲目科属种”，动物界有那么多动物，比如都是猫科动物，难道老虎和狮子还不配拥有个自己的类型吗，只能共用猫类型吗？要是想为老虎这个类型单独实现点东西，但不想为狮子也实现这个东西，共用猫类型就不行了！这样看起来，接受类型爆炸挺好，类型完整，也没几个类型，程序大小允许就可以，相比于动不动就异步的task、协程，只要不是大规模类型爆炸，可以忍。而类型统一就会造成一种“类型丢失”，它的不良影响发生在后续为Duck添加其它行为时，这些行为并非所有Duck都需要的时候。比如为绿头鸭实现捕猎，为橡皮鸭实现电动，它们不再是所有鸭子都应有的行为，已有点不再适合使用新策略扩展（可不是所有扩展的行为都是鸭子通用型的Swim、Display，策略模式只拣好的说），但动多态却因“类型丢失”而不知所措，这其实是个难处理的点，本质是为了减少类型爆炸而采用动多态统一类型的牺牲。

/// 静多态可以直接别名  
type MallardDuck = Duck<...>;  
type RubberDuck = Duck<...>;  
type DecoyDuck = Duck<...>;  
  
  
/// 动多态因“类型丢失”，只能使用NewType，并在NewType中约束DynamicDuck。  
/// 那这样，类型还是难免爆炸了啊！  
struct MallardDuck(DynamicDuck);  
struct RubberDuck(DynamicDuck);  
struct DecoyDuck(DynamicDuck);  
  
/// 仅为绿头鸭MallardDuck实现捕猎  
impl MallardDuck {  
    fn hunt(&self) {  
        ...  
    }  
}  

动多态策略模式再往下写很可能就开始坏味道了。为了解决这个问题，各种奇招就来了，如不管三七二十一，先把捕猎行为塞进Duck中，管其它鸭子会不会错用呢；或者，为橡皮鸭RubberDuck、木头鸭WoodDuck也实现个假的捕猎，这样“捕猎”就又符合新的策略了，又能使用策略模式了；又或者，再来次继承把绿头鸭子类化吧，然后单独给绿头鸭实现捕猎。。然而新类型MallardDuck一方面与动多态复合类型的Duck意义有冲突，不得不在文档中留下一句提醒使用者：“如果想用MallardDuck，请勿使用DynamicDuck构建，而是使用更具体的MallardDuck！”；另一方面，其它类型的Duck也需要子类化吗，若是的话岂不是又免不了类型爆炸了！策略模式这时正失去优雅的光环，它还是那个妙不可言的“策略模式”吗？

Rust语言，则可以静多态一路走到黑，`Duck<F, Q>`类型当参数时一直泛型约束使用下去。这样看起来，静多态是一种挺好的应对策略模式后续变化的解决方案。Rust还有一种方式，可以终止这种“一直”，就是将有限的静多态类型通过enum和类型统一起来，然后再使用时就不必继续用泛型了，用这个enum和类型就好了。这是个好方法，但也有个弊端，enum和类型终止了模块之外的“扩展性”！在模块之外，再也无法为模块内的enum和类型扩展其它Duck实现，而动多态和一直泛型约束的静多态，则仍不失模块外的扩展性。

策略模式还有个问题，值得探讨，Duck也会飞，也会呱呱叫了，那有没有必要为Duck也实现Fly、Quack特型呢？

/// 有没有必要为Duck实现Fly/Quack trait？  
impl<F, Q> Fly for Duck<F, Q>   
where  
    F: Fly,  
    Q: Quack,  
{  
    fn fly(&self) {  
        self.fly_behavior.fly();  
    }  
}  
  
impl<F, Q> Quack for Duck<F, Q>  
where  
    F: Fly,  
    Q: Quack,  
{  
    fn quack(&self) {  
        self.quack_behavior.quack();  
    }  
}  

这是个令人迷惑的选项，个人很讨厌这种“都可以”的选项，让人迟迟下不了决策。很多人从“应该不应该”的角度出发，会得到“应该”的答案，Duck应该会飞，所以为Duck实现了Fly特型，后面就可以用Fly来特型约束了。其实，若实现了，就像是另外一个设计模式——装饰器模式了。但我不建议普通的策略模式这样实现，将Fly和Quack组合起来的Duck，不再是飞行策略实现的一种变体，要是RubberDuck也能因满足Fly特型约束，再次充当Duck自己的“翅膀”F，组合成一个新Duck，那这是什么Duck？闹笑话了，一向以“严格”著称的Rust可不喜欢这样做。看起来Duck会飞，和飞行策略的Fly特型有所不同，读者可自行感受，那如何约束Duck，让别人知道Duck也是可飞行的一个类型呢？可以使用AsRef，让鸭子实现`AsRef<F: Fly>`，意为“Duck拥有飞行的策略”，鸭子自然也会飞，能做所有会飞的类型可以做的事情。

fn fly_to_do_sth<T, F>(fly_animal: &mut T)   
where  
    T: AsRef<F>,  
    F: Fly,  
{  
    // Duck也可以作为fly_animal来执行此函数了  
}  

注意，这里AsRef跟Deref的区别。AsRef可以实现多次，到不同类型的借用转换，比如Duck同时AsRef<F: Fly>和AsRef<Q: Quack>；而Deref只能实现一次到一个主Target的类型转换，而Fly和Quack无论哪个行为，明显都不足以让Duck为其实现Deref，它的父类动物结构，才值得Duck使用Deref。

## 小结

初识策略模式时，觉得妙不可言，但它其实没提策略模式那逐渐不可控的后续演化，源于为策略模式的复合类型Duck扩展行为时，并非所有Duck都该有这些扩展行为了，它们很可能是某些鸭子独有的，主要原因是动多态造成了“类型丢失”，而解决办法还没法令人满意！因此，策略模式适合后续不再演化的场景。能应对后续演化的，还得是类型完整的静多态思路。

编程的一大挑战就是为了应对变化，开发者知道的招式变化越多，应对的就越从容，使用看起来正确实际上却会逐渐失控的招式，只会味道越来越坏。变化就是“可扩展性”，谈到“可扩展性”，面向对象说这个我熟，“可扩展性”就是面向对象的目标之一啊！先别轻信，完美应对变化可不容易，即便资深的面向对象专家，都不敢说他写的每个东西都真能满足“单一职责”。。单一职责的足够“原子化”吗？面向对象思想有个老毛病，就是不够具体，让人抓不到，又让人以为抓到了，实际上是面向对象规定的东西，包括它的评论、解释大都泛泛而谈，没有一个度，很难意见统一。
```


## 原型法

```
策略模式，要是为复合类型也实现trait，就类似装饰器模式，因为装饰器无论是内部委托成员，还是外部装饰器自己，都得实现同一个名为Decorate的trait，就是为了让它们可以相互嵌套组合：

trait Decorate {  
    fn decorate(&mut self, params...);  
}  
  
/// 一个静多态的装饰器  
struct SomeDecorator<D: Decorate> {  
    delegate: D,    // 必要的委托  
    ...  
}  
  
/// 还得为Decorator自己实现Decorate特型  
impl<D: Decorate> Decorate for SomeDecorator<D> {  
    fn decorate(&mut self, params...) {  
        // 1. SomeDecorator itself do sth about params   
        self.do_sth_about_params(params...); // 这是真正要装饰的实现  
        // 2. then turn self.delegate  
        self.delegate.decorate(params...);    // 这一句都相同，1、2步的顺序可互换  
    }  
}  
  
/// 另一个装饰器  
struct AnotherDecorator<T: Decorate> {  
    delegate: T,  
    ...  
}  
  
impl<D: Decorate> Decorate for AnotherDecorator<D> {  
    fn decorate(&mut self, params...) {  
        // 1. AnotherDecorator itself do sth about params   
        self.do_sth_about_params(params...);  
        // 2. then turn self.delegate  
        self.delegate.decorate(params...);    // 这一句都相同  
    }  
}  
  
/// 必要的终结型空装饰器  
struct NullDecorator;  
  
impl Decorator for NullDecorator { /*do nothing*/ }  
  
/// 使用上  
let d = SomeDecorator::new(AnotherDecorator::new(NullDecorator));  
d.decorate();  

SomeDecorator/AnoterDecorator是真正的装饰器，会有很多个，功能各异，每个Decorator所包含的相应的结构可能也不同。装饰器在使用上，就像链表一样，一个处理完之后，紧接着下一个节点再处理，它把链表结构包含进了装饰器的结构里面，并用接口/trait来统一类型。上述实现有重复代码，就是调用委托的装饰方法，还能继续改进：

/// 装饰的其实是一个处理过程  
trait Handle {  
    fn handle(&mut self, params...);  
}  
  
trait Decorate {  
    fn decorate(&mut self, params...);  
}  
  
/// 装饰器的终结  
struct NullDecorator;  
  
impl Decorate for NullDecorator {  
    fn decorate(&mut self, params...) {  
        // do nothing  
    }  
}  
  
/// 通用型装饰器，像是链表节点串联前后2个处理器节点  
struct Decorator<D: Decorate, H: Handler> {  
    delegate: D,  
    handler: H,   // 这又是个干净的模板方法，将变化交给子类  
}  
  
/// 通用装饰器本身也得实现Decorate特质，可以作为另一个装饰器的D  
impl<D: Decorate, H: Handler> Decorate for Decorator<D, H> {  
    fn decorate(&mut self, params...) {  
        // 这两步可互换  
        self.handler.handle(params);  
        self.delegate.decorate(params);  
    }  
}  
  
/// 下面的处理器只关注处理器自己的实现就好了  
struct SomeHandler { ... };  
  
impl Handler for SomeHandler { ... }  
  
struct AnotherHandler { ... };  
  
impl Handler for AnotherHandler { ... }  
  
/// 使用上  
let d = Decorator {  
    delegate: Decorator {  
        delegate: NullDecorator,  
        handler: AnotherHandler,  
    },  
    handler: SomeHandler,  
};  
d.decorate(params...);  

可以看出，装饰器很像链表，emm...大家都知道链表在Rust中较复杂，那链表有多复杂，装饰器就有多复杂。上面的静多态实现也是不行的，不同的装饰器组合，就会产生不同的类型，类型可能随着Handler类型数目增加呈其全排列阶乘级类型爆炸，忍不了，必须得换用指针。装饰器模式，Rust实现起来不如传统面向对象，面向对象天然动多态，且Decorator继承可以让D、H两部分合为一体，让H也成装饰类的一个虚函数，都在this指针访问范围内，简单一些。而Rust将装饰器拆解成了链表型，将装饰器的底层结构还原了出来，确实装饰器可以用链表串联起各个处理器一个接一个地调用，效果一样的。只是面向对象技巧隐藏了链表的细节。

不过Rust有个很牛逼的装饰器，就是迭代器的map、step_by、zip、take、skip这些函子，它们可以随意串联组合调用，本质就是装饰器，只不过仅限于用在迭代器场景。如果装饰器能这样实现，能惰性求值，也能够编译器內联优化，就太强了。不过，各个装饰器功能不同，恐怕不能像迭代器函子那样都有清晰的语义，因此没有统一的装饰器库。不过装饰器实现时，肯定可以借鉴迭代器的函子思路。这样一来的话，Rust的装饰器又丝毫不弱于传统面向对象的了。而且，高，实在是高，妙，实在是妙！

/// 以下仅作摘选，让大家一窥迭代器函子的装饰器怎么玩的  
pub trait Iterator {  
    type Item;  
  
    // Required method  
    fn next(&mut self) -> Option<Self::Item>;  
  
    // Provided methods  
    // 像下面这样的函数还有76个，每个函数都映射到一个具体的装饰器，它们都返回一个装饰函子impl Iterator<Item = Self::Item>  
    // 装饰器函数基本都定义完了，未来无法扩展？还记得原型法吗，为所有实现了Iterator的类型实现IteratorExt  
    // 仅挑选一个step_by作为案例  
    #[inline]  
    #[stable(feature = "iterator_step_by", since = "1.28.0")]  
    #[rustc_do_not_const_check]  
    fn step_by(self, step: usize) -> StepBy<Self>  
    where  
        Self: Sized,  
    {  
        StepBy::new(self, step)  
    }  
}  
  
/// StepBy装饰器，如第一种实现那样的写法  
pub struct StepBy<I> {  
    iter: I,    // 装饰器的delegate  
    step: usize,  
    first_take: bool,  
}  
  
/// 再为StepBy<I>实现Iterator  
impl<I> Iterator for StepBy<I>  
where  
    I: Iterator,  
{  
    type Item = I::Item;  
  
    #[inline]  
    fn next(&mut self) -> Option<Self::Item> {  
        self.spec_next()  
    }  
}  
  
// 使用上，有别于传统装饰器模式从构建上去串联，这是利用返回值链式串联，顿时清晰不少  
vec![1, 2, 3].iter().skip(1).map(|v| v * 2);  

## 小结

至此，模板方法的变化告一断落。之前，有人说Rust不支持面向对象，导致Rust不好推广，实际上并不是，哪个OO设计模式Rust实现不了，还更胜一筹。因此，并非Rust不支持面向对象！有些设计模式，Rust天生也有，如：

- 单例模式：其实单例模式如果不是为了懒加载，跟使用全局变量没啥差别；如果为了懒加载，那`lazy_static`或者`once_cell`就够用。（补充：标准库已经标准化成`OnceLock`了）
    
- 代理模式：NewType模式作代理挺好；或者原型法“原地”扩展代理行为
    
- 迭代器模式：Rust的迭代器是我见过最NB的迭代器实现了
    
- 状态机模式：Rust语言官方文档中的NewType+enum状态机模式，这种静多态的状态机非常严格，使用上都不会出错，所有状态组合还可以用enum统一起来，比面向对象的状态机模式要好
    

还有一些设计模式，跟其它模式很像，稍加变化：

- 适配器模式：同代理模式差别不大，很可能得有自己的扩展结构，然后得有额外“兼容处理”逻辑来体现“适配”
    
- 桥接模式：就是在应用策略模式
    
- 过滤器模式：就是在应用装饰器模式
```


