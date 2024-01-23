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