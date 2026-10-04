快速回顾一下Rust的Trait

编写一个Rust trait，如下：

```rust
pub trait MyTrait {    fn some_method(&self) -> String;}
```

  

我们定义一个Struct类型来实现MyTrait：

```rust
struct MyStruct;impl MyTrait for MyStruct {    fn some_method(&self) -> String {        "Hi from some_method!".to_string()    }}
```

  

也可以在我们自有的类型上实现我们不拥有的Trait，或者在不拥有的类型上实现我们拥有的Trait——但不能两者都实现！不能这么做的原因是因为Trait一致性。要确保我们不会意外地有冲突的trait：

```rust
// 在MyStruct上实现Into<T>，一个我们不拥有的traitimpl 
Into<String> for MyStruct {    fn into(self) -> String {        "Hello world!".to_string()    }}

// 为一个我们不拥有的类型实现MyTraitimpl MyTrait for 
String {    fn some_method(&self) -> String {        self.to_owned()    }}

// 这样是错误的
impl Into<String> for &str {    fn into(self) -> String {        self.to_owned()    }}
```

  

一种常见的解决方法是创建一个新类型模式——也就是说，用一个单字段元组结构体来封装我们想要扩展的类型。

```rust
struct MyStr<'a>(&'a str);// 注意，实现From<T>也实现了Into<T>impl<'a> From<MyStr<'a>> for String {    fn from(string: MyStr<'a>) -> String {        string.0.to_owned()    }}
```

  

如果多个trait有相同的方法名，需要手动声明从哪个trait实现中调用方法：

```rust
pub trait MyTrait {    fn some_method(&self) -> String;}pub trait MyTraitTwo {    fn some_method(&self) -> i32;}struct MyStruct;impl MyTrait for MyStruct {    fn some_method(&self) -> String {        "Hi from some_method!".to_string()    }}impl MyTraitTwo for MyStruct {    fn some_method(&self) -> i32 {        42    }}fn main() {    let my_struct = MyStruct;    println!("{}", MyTraitTwo::some_method(&my_struct));}
```

  

有时，可能希望Trait中的方法能够拥有默认实现，我们可以通过简单地在trait中定义方法来实现这一点。

```rust
trait MyTrait {    fn some_method(&self) -> String {        "Boo!".to_string()    }}
```

Trait也可以用其他Trait作为边界限制，以std::error:: error特性为例：

```rust
trait Error: Debug + Display {    // .. 如果需要，可以重新实现这里提供的方法}
```

这里，我们显式地告诉编译器，我们的类型必须在实现Error之前实现Debug和Display两个trait。

  

  

标记Trait

标记Trait被用作“标记”，当为一个类型实现标记Trait时，可以维持某些特性。它们没有方法或特定的属性，通常用于确保编译器的某些行为。

特别是这两个标记Trait，对我们来说非常重要：Send和Sync。手动实现Send和Sync是不安全的——这是因为需要手动确保其安全实现，Unpin也是另一个例子。

除此之外，标记Trait(一般来说)也是自动实现的。如果一个结构体的字段都实现了trait，那么这个结构体本身也会实现trait。例如：

- 如果结构体中的所有字段类型都是Send，那么编译器会自动将该结构体标记为Send，而不需要手动输入。
    
- 如果结构体的一个字段实现了Clone，而另一个字段没有，那么结构体就不能再派生Clone了，但是可以通过在Arc或Rc中包装相关类型来解决这个问题。
    

为什么标记Trait在Rust中很重要？

Rust中的标记特征构成了生态系统的核心，并允许我们提供在其他语言中无法实现的保证。以Send类型为例，我们可以确保跨线程发送类型总是安全的。这使得并发问题更容易处理。标记Trait也会影响其他事情：

- Copy Trait需要通过执行逐位复制来复制内容(尽管这需要Clone)。尝试按位复制指针只返回地址！这也是String不能被复制而必须被克隆的原因：Rust中的String是智能指针。
    
- Pin Trait允许我们将一个值“固定”到内存中的一个静态位置
    
- Sized trait允许我们在编译时将类型定义为具有固定大小的尺寸——这已经在大多数类型中自动实现了
    

还有一些标记Trait，比如?Sized、!Send和!Sync。与size, Send和Sync相比，它们是相反的Trait边界，做完全相反的事情：

- ?Sized：动态调整类型大小
    
- !Send：告诉编译器一个对象绝对不能发送给其他线程
    
- !Sync：告诉编译器对象的引用绝对不能在线程之间共享
    

Trait Object和动态调度

动态分派本质上是在运行时选择使用多态函数的哪个实现的过程。虽然Rust出于性能原因倾向于静态分派，但通过trait object使用动态分派也有好处。

使用trait object最常见的模式是Box dyn MyTrait，我们需要将trait object包装在Box中，以使其实现Sized的trait。因为我们将多态过程移到了运行时，编译器无法知道类型的大小。将类型包装在指针中(或将其“装箱”)将其放在堆中而不是栈中。

  

```rust
struct MyStruct {     my_field: Box<dyn MyTrait>}// this works!fn my_function(my_item: Box<dyn MyTrait>) {     // .. some code here}// this doesn't!fn my_function(my_item: dyn MyTrait) {     // .. some code here}trait MySizedTrait: Sized {    fn some_method(&self) -> String {        "Boo!".to_string()    }}// 由于大小限制而无法编译的非法结构体struct MyStruct {    my_field: Box<dyn MySizedTrait>}
```

动态分派的主要优点是你的函数不需要知道具体的类型，只要类型实现了trait，你就可以把它作为trait object使用。从用户的角度来看，编译器并不关心底层的具体类型是什么——只关心它实现了trait。

缺点是需要确保trait object的安全性。安全性需要满足的条件包括：

- 类型不能是 Self: Sized
    
- 类型必须在函数参数中使用某种类型的“self”(无论是&self, self, mut self等)
    
- 类型不能返回Self
    

这源于这样一个事实：通过将分派移到运行时，编译器无法猜测类型的大小——Trait Object在编译时没有固定的大小。这也是为什么我们需要像前面提到的那样将动态分派的对象封装在Box中，并将它们放在堆上。因此，应用程序的性能也会受到影响——当然，这取决于正在使用多少动态分派的对象以及它们有多大。

  

  

结合Trait和泛型

我们可以毫不费力地编写这样一个实现泛型的结构体：

```rust
struct MyStruct<T> {    my_field: T}
```

然而，为了能够将我们的结构体与来自其他容器的类型一起使用，我们需要确保我们的结构体能够保证某些行为。这就是我们添加trait边界的地方：一个类型必须满足条件才能使其编译。你可能会发现一个常见的特征绑定是Send + Sync + Clone：

```rust
struct MyStruct<T: Send + Sync + Clone> {    my_field: T}
```

作为一个使用泛型Trait更复杂的例子，你可能偶尔需要为自己的类型重新实现，以Axum的FromRequest trait为例：

```rust
use axum::extract::State;use axum::response::IntoResponse;trait FromRequest<S>   where S: State    {    type Rejection: IntoResponse;    fn from_request(r: Request, _state: S) -> Result<Self, Self::Rejection>;}
```

这里我们还可以通过使用where子句来添加trait边界。这个特性只是告诉我们S实现了State。但是，State还要求内部对象为Clone。通过使用复杂的trait边界，我们可以创建大量使用trait的框架系统，从而能够实现一些人所说的“trait魔法”。看一下这个trait边界的例子：

```rust
use std::future::Future;struct MyStruct<T, B> where   B: Future<Output = String>,   T: Fn() -> B{    my_field: T}#[tokio::main]async fn main() {    let my_struct = MyStruct { my_field: hello_world };    let my_future = (my_struct.my_field)();    println!("{:?}", my_future.await);}async fn hello_world() -> String {    "Hello world!".to_string()}
```

上面的单字段结构体存储了一个函数闭包，该闭包返回impl Future Output = String>，我们将hello_world存储在其中，然后在主函数中调用它。

像这样结合Trait和泛型是非常强大的，有效利用这一点的一个用例是HTTP框架。例如，Actix Web有一个名为Handler<Args>的trait，它接受许多参数，调用自己，然后有一个名为call的函数产生一个Future：

```rust
pub trait Handler<Args>: Clone + 'static {     type Output;     type Future: Future<Output = Self::Output>;     fn call(&self, args: Args) -> Self::Future;}
```

这样我们就可以将这个Trait扩展为一个处理程序函数。我们可以告诉web服务，我们有一个函数，它有一个内部函数，一些参数和Responder (Actix web的HTTP响应Trait)：

```rust
pub fn to<F, Args>(handler: F) -> Route where    F: Handler<Args>,    Args: FromRequest + 'static,    F::Output: Responder + 'static {    // .. the actual function  code here}
```

## Rust 中使用泛型函数的小技巧

每次调用泛型函数时，编译器会为每种类型组合生成一个实现，这可能会产生大量代码。
通过将大部分代码移到一个内部函数中，只生成一次大块代码，而多次生成较小的类型转换代码。
Rust 标准库中就大量采用了这种技巧，例如:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub fn with_extension<S: AsRef<OsStr>>(&self, extension: S) -> PathBuf {
    self._with_extension(extension.as_ref())
}

fn _with_extension(&self, extension: &OsStr) -> PathBuf {
    let self_len = self.as_os_str().len();
    let self_bytes = self.as_os_str().as_encoded_bytes();

    let (new_capacity, slice_to_copy) = match self.extension() {
        None => {
            // Enough capacity for the extension and the dot
            let capacity = self_len + extension.len() + 1;
            let whole_path = self_bytes.iter();
            (capacity, whole_path)
        }
        Some(previous_extension) => {
            let capacity = self_len + extension.len() - previous_extension.len();
            let path_till_dot = self_bytes[..self_len - previous_extension.len()].iter();
            (capacity, path_till_dot)
        }
    };

    let mut new_path = PathBuf::with_capacity(new_capacity);
    new_path.as_mut_vec().extend(slice_to_copy);
    new_path.set_extension(extension);
    new_path
}
```