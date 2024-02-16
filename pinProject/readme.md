# Rust为什么需要Pin、Unpin？

使用异步Rust库通常很容易，这就像使用普通的Rust代码一样，使用.async或.await。但是编写自己的异步库可能很困难。有一些晦涩难懂的语法，比如T: Unpin和Pin<&mut Self>。

## 自引用是不安全的

Pin的存在是为了解决一个非常具体的问题:自引用数据类型，即具有指向自身的指针的数据结构。例如，二叉搜索树可能具有自引用指针，这些指针指向同一结构中的其他节点。

自引用类型可能非常有用，但它们很难保证内存安全。为了了解原因，让我们使用这个带有两个字段的类型作为示例，一个名为val的i32类型的字段和一个名为pointer的指向i32的指针。

![](../learning/src/objInfo/assets/Pasted%20image%2020240216113844.png)
到目前为止，一切正常。指针字段指向内存地址A中的val字段，其中包含一个有效的i32。所有指针都是有效的，也就是说，它们指向的内存确实编码了正确类型的值(在本例中是i32)。

但是Rust编译器经常在内存中移动值。例如，如果将这个结构体传递给另一个函数，它可能会被移动到不同的内存地址，或者我们使用Box把它放在堆上；或者，如果这个结构体在Vec<‘MyStruct>中，并且我们将更多的值压入，Vec可能会超出其容量，需要将其元素移动到一个新的更大的缓冲区中。

![](../learning/src/objInfo/assets/Pasted%20image%2020240216113934.png)
当我们移动它时，结构体的字段会改变它们的地址，但不会改变它们的值。所以指针字段仍然指向地址A，但是地址A现在没有一个有效的i32。原来在那里的数据被移到了地址B，而其他一些值可能被写到了那里，现在指针是无效的。

这很糟糕——在最好的情况下，无效指针会导致崩溃，在最坏的情况下，它们会导致可攻击的漏洞。我们应该非常小心地记录这种类型，并告诉用户在移动后更新指针。
## Unpin 和 !Unpin

回顾一下，所有Rust类型都分为两类：

1，可以安全地在内存中移动的类型。这是默认的，是常态。例如，这包括像数字、字符串、bool这样的原语，以及完全由它们组成的结构体或枚举。大多数类型都属于这一类!

2，自引用类型，在内存中移动是不安全的。这是非常罕见的，一个例子是一些Tokio内部内部的侵入式链表，另一个例子是大多数实现Future，同时也借用了数据的类型。

类别1中的类型在内存中移动是完全安全的，移动指针不会使它们失效。但是，如果在类别2中移动一个类型，那么指针就会失效，并可能得到未定义的行为，正如我们之前看到的那样。在早期的Rust版本中，你必须非常小心地使用这些类型，不要移动它们，或者如果你移动了它们，使用不安全并更新所有的指针。但是从Rust 1.33开始，编译器可以自动找出任何类型属于哪个类别，并确保您只安全地使用它。

类别1中的任何类型都自动实现了一个称为Unpin的特殊Trait。奇怪的名字，但它的意思很快就会清楚。同样，大多数“正常”类型实现了Unpin，因为它是一个自动实现的Trait(像Send或Sync或Sized)，所以你不必担心自己实现它。如果你不确定是否可以安全地移动类型，只需在文档中检查它是否实现了Unpin即可。

类别2中的类型是创造性地命名为!Unpin(!在trait中意味着“不实现”)。为了安全地使用这些类型，不能使用常规指针进行自引用。相反，我们使用特殊的指针来“固定”它们的值，确保它们不能被移动，这正是Pin类型所做的。

![](../learning/src/objInfo/assets/Pasted%20image%2020240216114403.png)

Pin封装指针并阻止其值移动，唯一的例外是如果值包含Unpin，那么我们就知道移动是安全的。现在我们可以安全地编写自引用结构了！这一点非常重要，因为正如上面所讨论的，许多future都是自引用的，我们需要它们来实现async/await。

## 使用Pin

现在我们理解了Pin存在的原因，以及为什么我们的Future poll方法有一个固定的&mut self到self，而不是一个常规的&mut self。那么让我们回到之前的问题：我需要一个指向内部Future的固定引用。更一般地说：给定一个固定的结构体，我们如何访问它的字段？

解决方案是编写帮助函数，为你提供对字段的引用。这些引用可能是普通的Rust引用，比如&mut，或者它们也可能是固定的。你可以选择你需要的任何一个。这就是所谓的投影：如果你有一个固定的结构体，你可以编写一个投影方法，让你访问它的所有字段。

投影实际上就是数据与Pin类型互相转换，例如，我们从Pin<&mut self>中获得start: Option<’Duration>字段，并且我们需要将future: Fut放入Pin中，以便我们可以调用其poll方法)。如果你阅读Pin方法，你会发现如果它指向一个Unpin值，它总是安全的，否则就要求使用Unsafe。
```rust
// 将数据放入Pin  
pub        fn new          <P: Deref<Target:Unpin>>(pointer: P) -> Pin<P>;  
pub unsafe fn new_unchecked<P>                     (pointer: P) -> Pin<P>;  
  
// 从Pin获取数据  
pub        fn into_inner          <P: Deref<Target: Unpin>>(pin: Pin<P>) -> P;  
pub unsafe fn into_inner_unchecked<P>                      (pin: Pin<P>) -> P;
```
## 用pin-project库

对于结构体中的每个字段，你必须选择是否应该固定其引用。默认情况下，应该使用普通引用，因为它们更容易、更简单。但是如果你知道你需要一个固定的引用——例如，因为你想调用.poll()，它的接收者是Pin<&mut Self>——那么你可以用#[Pin]来做。

例子如下：
```rust
/*

! 解决异步编程的引用问题 pin 和 unpin

*/

  

#[pin_project::pin_project]

pub struct TimedWrapper<Fut: Future> {

    // 对于每个字段，我们需要选择是返回对该字段的未固定(&mut)引用

    // 还是固定(Pin<&mut >)引用。

    // 默认情况下，它是未固定的

    start: Option<Instant>,

    // 此属性选择固定引用

    #[pin]

    future: Fut,

}

  

impl<Fut: Future> Future for TimedWrapper<Fut> {

    type Output = (Fut::Output, Duration);

  

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {

        // 这将返回一个具有所有相同字段的类型，所有相同的类型，

        // 除了用#[pin]定义的字段将被固定。

        let mut this = self.project();

  

        // 调用内部poll，测量花了多长时间。

        let start = this.start.get_or_insert_with(Instant::now);

        let inner_poll = this.future.as_mut().poll(cx);

        let elapsed = start.elapsed();

  

        match inner_poll {

            // 内部Future需要更多的时间，所以这个Future也需要更多的时间

            Poll::Pending => Poll::Pending,

            // Success!

            Poll::Ready(output) => Poll::Ready((output, elapsed)),

        }

    }

}

  
  

// 最后，我们的目标完成了——我们在没有任何不安全代码的情况下完成了这一切。

fn main() {

    println!("Hello, world!");

}
```

如果Rust类型具有自引用指针，则不能安全地移动它。毕竟，移动并没有更新指针，所以它们仍然指向旧的内存地址，所以它们是无效的。

Rust可以自动判断哪些类型可以安全移动(并将自动为它们实现Unpin trait)。如果你有一个Pin的指针指向某些数据，Rust可以保证不会发生任何不安全的事情。这一点很重要，因为许多Future类型都是自引用的，所以我们需要Pin来安全地轮询Future。你可以使用pin-project crate来简化操作。
