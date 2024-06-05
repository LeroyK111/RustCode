# 闭包


闭包是Rust的一个强大特性，它允许函数捕获上下文变量。


## 捕获上下文变量

闭包是可以捕获其周围环境并使用其作用域之外变量的函数。|…|语法用于定义闭包。将闭包视为一个可以捕捉其周围环境快照的摄像机。它们可以在快照时“记住”变量值，并在稍后调用时使用它们。
```rs
let x = 3;
let closure = |num| num * x;
let result = closure(2);
```
本例中的闭包从环境中捕获变量' x '，并将其乘以输入' num '。


## 闭包的灵活特质

闭包可以通过三种方式捕获变量，具体取决于所需的功能：

1，通过引用捕获：允许你查看和使用变量，但不允许你修改或移动它。

2，通过可变引用捕获：允许使用、修改和重新排列变量但不能删除变量。

3，按值捕获：允许你提取变量并根据需要使用它，包括将其移动到另一个位置。

```rs
let x = 3;
let by_ref = || println!("{}", x); // 通过引用捕获x
let mut y = 5;
let by_mut_ref = || { y += 1; println!("{}", y); }; // 通过可变引用捕获y
let by_value = move || println!("{}", x); // 按值捕获x
```

## 闭包作为输入参数

当一个闭包被用作输入参数时，它的完整类型必须用以下特征之一进行注释：

1，Fn：闭包通过引用使用捕获的值。(&T)

2，FnMut：闭包通过可变引用使用捕获的值。(&mut T)

3，FnOnce：闭包使用捕获的值。(T)
```rs
fn apply<F: Fn(i32) -> i32>(f: F, num: i32) -> i32 {
    f(num)
}

let double = |x| x * 2;
let result = apply(double, 4);
```

## 匿名类型

闭包具有匿名类型，在用作函数参数时必须与泛型一起使用。
```rs
fn call_twice<F>(closure: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    closure(value) + closure(value)
}

let add_five = |x| x + 5;
let result = call_twice(add_five, 10);
```
使用' where '子句的泛型来基于闭包的行为约束闭包类型，允许我们在不知道其确切类型的情况下与它进行交互。

## 输入函数：将函数作为参数传递

闭包和函数都可以用作参数。在声明以闭包为参数的函数时，任何满足闭包特征边界的函数都可以作为参数传递。
```rs
fn square(x: i32) -> i32 {
    x * x
}

let result = apply(square, 4); // 传递一个函数而不是闭包

```

## 闭包作为返回值

可以将闭包作为输出参数返回，但由于匿名闭包类型在定义上是未知的，因此必须使用impl Trait来这样做。

以下是返回闭包的有效特征：

1，Fn

2，FnMut

3，FnOnce

```rs
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

let add_five = create_adder(5);
let result = add_five(10);
```
本例中的create_adder函数返回一个闭包，将x添加到其输入中。闭包使用' move '关键字按值捕获x，该函数返回一个带有impl Fn(i32) -> i32的闭包。

闭包是Rust中的一个强大且可适应的特性，允许你从环境中捕获和使用变量。

# Rust：临时变量的生命周期

## 临时变量

下面是一个没有上下文的Rust语句，它使用了一个临时字符串变量：
```
f(&String::from('🦀'));
```

这个临时字符串存在多长时间？如果我们设计Rust语言，我们基本上可以从两种选项中选择：

在调用f之前，字符串会被立即删除。


字符串只会在调用f之后被删除。


如果我们使用选项1，上面的语句将总是导致借用检查错误，因为我们不能让f借用已经不存在的东西。

因此，Rust选择了选项2：首先分配String，然后将对它的引用传递给f，只有在f调用返回后才删除临时String。


## 在let语句中

现在有一个稍微难一点的问题：
```
let a = f(&String::from('🦀'));
…
g(&a);
```

再问一次：临时字符串变量的生命周期是多长？

字符串在let语句的末尾被删除，在f返回之后，但在g被调用之前。



在调用g之后，字符串将与a同时被丢弃。



这一次，选项1可能有效，取决于f的签名。如果f被定义为fn f(s: &str) -> usize，那么在let语句之后立即删除String是完全可以的。

然而，如果f被定义为fn f(s: &str) -> &[u8]，那么a将从临时String变量中产生借用，因此如果我们将a保留更长时间，我们将得到一个借用检查错误。

对于选项2，它在两种情况下都可以很好地编译，但是我们可能会保留一个临时变量比必要的存活时间更长，这可能会浪费资源或导致微妙的错误(例如，当MutexGuard比预期的更晚被丢弃时，会出现死锁)。

在选项1和选项2之间，Rust选择了选项1：在let语句末尾删除临时变量。手动将String移动到单独的let语句中以使其保持更长的生命周期。
```
let s = String::from('🦀');
let a = f(&s);
…
```



## 在嵌套调用中

再看一个更复杂的：
```
g(f(&String::from('🦀')));
```

同样，有两种选择：

在调用f之后，但在调用g之前，字符串被删除。



该字符串将在语句结束时删除，因此在调用g之后。



该代码段与前一个代码段几乎相同：将对临时String变量的引用传递给f，并将其返回值传递给g。不过，这一次，使用了单个的嵌套调用表达式语句。

根据f的签名，选项1可能起作用，也可能不起作用，选项2可能使临时变量的生命周期存在的时间比必要的长。

选项1会使像
`String::from('🦀').as_bytes().contains(&0x80)`
这样简单的东西也不会通过编译，因为字符串会被丢弃在as_bytes[f]之后，在contains[g]之前。

因此，Rust选择了选项2：不管f的签名是什么，String都保持存活，直到语句结束，直到调用g之后。


## 在if语句中

现在让我们来看一个简单的if语句：
```
if f(&String::from('🦀')) {
    …
}
```

同样的问题：什么时候删除临时字符串变量？

在if语句的条件求值之后，但在if语句体执行之前(即在{处)。



在if函数体之后(即在}处)。



在这种情况下，没有理由在if语句体期间保持临时变量的存活。该条件的结果是一个布尔值(只有true或false)，根据定义，它不借用任何东西。

所以，Rust选择了选项1。

一个有用的例子是使用Mutex::lock，它返回一个临时的MutexGuard，当它被丢弃时将解锁互斥锁：
```
fn example(m: &Mutex<String>) {
    if m.lock().unwrap().is_empty() {
        println!("the string is empty!");
    }
}
```

这里，m.lock().unwrap()中的临时变量MutexGuard在.is_empty()之后被删除，这样在println语句期间互斥量就不会不必要地保持锁定状态。

## 在if let语句中

但是，if let和match的情况不同，因为我们的表达式不一定求值为布尔值：
```
if let … = f(&String::from('🦀')) {
    …
}
```

还是有两种选择：

在模式匹配之后，在if let的主体之前(即在{处)删除字符串。



在if let语句体之后(即在}处)删除该字符串。



这一次，我们有理由选择第二种而不是第一种。在if let语句或match这种模式匹配语句中，借用某些东西是很常见的。

因此，在这种情况下，Rust选择了选项2。例如，如果我们有一个Mutex<Vec<T'>>类型的vec，下面的代码编译得很好：
```
if let Some(x) = vec.lock().unwrap().first() {
    // 互斥对象仍然被锁在这里
    // 因为我们从Vec中借用了x. (`x` 是 `&T`)
    println!("first item in vec: {x}");
}
```

我们从.lock().unwrap()中获得一个临时变量MutexGuard，并使用.first()方法借用第一个元素。这个借用在if let的整个主体中需要持续锁定，因此MutexGuard只在最后的}处被删除。

然而，在某些情况下，这并不是我们想要的。例如，如果我们不使用first，而是使用pop，它返回一个值而不是引用：
```
if let Some(x) = vec.lock().unwrap().pop() {
    // 互斥对象仍然被锁在这里
    // 这是不必要的，因为我们没有从Vec中借用任何东西。(“x” 是 “T”)
    println!("popped item from the vec: {x}");
}****
```

这可能会导致细微的错误或性能降低。

目前，解决方法是使用单独的let语句，将临时生命周期限制为let语句中：
```
let x = vec.lock().unwrap().pop(); // MutexGuard在此语句之后被删除
if let Some(x) = x {
    …
}
```


## Rust中闭包的优缺点？

闭包

闭包是匿名的内联函数，可以捕获和存储它们周围的环境(即它们作用域中的变量和值)。它们对于创建可以传递给高阶函数或存储在数据结构中的简短函数特别有用。闭包类似于lambdas或其他编程语言中的匿名函数。在Rust中，闭包是使用||语法定义的，后面跟着可选的输入参数列表、->返回类型(如果需要)和花括号{}内的代码块。

让我们看一个在Rust中演示闭包的例子。假设你想创建一个高阶函数，以闭包作为参数，并将其应用于两个数字：
```rust
fn apply<F>(f: F, a: i32, b: i32) -> i32 
where
    F: Fn(i32, i32) -> i32
{
    f(a, b)
}

fn main() {
    let add = |a: i32, b: i32| a + b;
    let mul = |a: i32, b: i32| a * b;

    println!("5 + 3 = {}", apply(add, 5, 3));
    println!("5 * 3 = {}", apply(mul, 5, 3))
}
```
执行cargo run，结果如下：
5 + 3 = 8
5 * 3 = 15
在这个例子中，我们定义了一个高阶函数apply，它接受一个闭包F和两个i32值作为输入。闭包是用Fn特征边界定义的，这意味着它可以是任何接受两个i32值并返回一个i32的函数或闭包。

在main函数中，我们定义了两个闭包，add和mul，分别执行加法和乘法。然后将这些闭包传递给apply函数，apply函数将闭包应用于输入的数字。

闭包提供了一种方便而简洁的方法来定义小型的一次性函数，这些函数可以作为参数传递给高阶函数(即接受其他函数作为输入的函数)或存储在数据结构中。在使用迭代器、异步编程以及需要传递一小块行为的任何场景时，它们特别有用。


闭包的优点

1，简洁

闭包为就地定义短函数提供了简洁的语法。例如，我们使用闭包“|a, b| a.len().cmp(&b.len())”直接在下面的代码示例中的sort_by方法调用中简明地表达排序逻辑。
```rust
fn main() {
    let mut words = vec!["banana", "apple", "orange", "grape"];

    words.sort_by(|a, b| a.len().cmp(&b.len()));

    println!("Sorted words: {:?}", words); // Output: Sorted words: ["apple", "grape", "banana", "orange"]
}
```

2，灵活

闭包可以从它们的环境中捕获变量，使它们更适合不同的用例。例如，闭包“|&x| x > threshold”捕获threshold变量，允许过滤逻辑适应不同的threshold值。
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let threshold = 3;

    let filtered_numbers: Vec<_> = numbers.into_iter().filter(|&x| x > threshold).collect();

    println!("Filtered numbers: {:?}", filtered_numbers); // Output: Filtered numbers: [4, 5]
}
```
3，封装

通过捕获变量，闭包可以在不需要额外数据结构的情况下维护状态。例如，内部闭包“ || { count += 1; count }”捕获count变量的值，将状态封装在闭包本身中。
```rust
fn main() {
    let mut count = 0;
    let mut counter = || {
        || {
            count += 1;
            count
        }
    }();

    println!("Counter: {}", counter()); // Output: Counter: 1
    println!("Counter: {}", counter()); // Output: Counter: 2
}
```
4，与高阶函数的兼容性

闭包可以作为参数传递给其他函数或存储在数据结构中，从而支持强大的抽象和模式。例如，apply_to_each函数(在下面的代码示例中)接受一个闭包作为参数，并将其应用于items切片中的每个元素。闭包“|x| *x *= 2”将数字向量中的每个元素加倍。
```rust
fn apply_to_each<T, F>(items: &mut [T], f: F)
where
    F: Fn(&mut T),
{
    for item in items {
        f(item);
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    apply_to_each(&mut numbers, |x| *x *= 2);

    println!("Doubled numbers: {:?}", numbers); // Output: Doubled numbers: [2, 4, 6, 8, 10]
}
```

闭包的缺点

1，有限的可重用性

闭包通常仅限于定义它们的代码的特定部分，因此不能在其他地方重用。

2，性能

由于捕获变量和动态分派，闭包有时会引入运行时开销。例如，在下面的示例中，闭包按值捕获large_data，将整个向量移动到闭包的环境中。这可能会导致性能开销，特别是如果向量很大，因为它需要在创建闭包时复制数据。在性能至关重要的情况下，使用带有显式参数的函数而不是捕获变量可能会更好。
```rust
fn main() {
    let large_data = vec![0; 1_000_000];
    let closure = move || {
        let _sum = large_data.iter().sum::<i32>();
    };

    // Use the closure
    closure();
}
```
3，复杂的生命周期

在捕获引用时，闭包可能会引入复杂的生命周期，使代码更难推理。在下面的例子中，我们试图返回一个闭包，该闭包捕获了对字符串的引用。但是，代码将无法编译，因为文本变量超出了作用域，并且闭包捕获的引用无效。这引入了生命周期的复杂性，使代码更难推理，并可能导致错误。
```rust
fn create_closure<'a>(input: &'a str) -> impl Fn() -> &'a str {
    move || input
}

fn main() {
    let closure;
    let output;

    {
        let text = String::from("Hello, world!");
        closure = create_closure(&text);
        output = closure();
    }

    println!("Output: {}", output);
}
```

总结

Rust中的闭包是匿名函数，可以从环境中捕获值。它们提供了一种富有表现力的、灵活的方式来定义短函数，可用于高阶函数的参数或存储在数据结构中。