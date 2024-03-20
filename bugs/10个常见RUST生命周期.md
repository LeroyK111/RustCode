# 10个常见的Rust生命周期误解 - 1

简而言之：变量的生命周期是它所指向的数据可以被编译器静态验证其在当前内存地址上的有效时间。

## 误解1：T只包含自己拥有的类型

这种误解更多的是关于泛型而不是生命周期，但在Rust中，泛型和生命周期是紧密交织在一起的，所以不可能只讨论一个而不讨论另一个。无论如何：

当刚开始学习Rust时，我们知道i32、&i32和&mut i32是不同的类型。我们还了解到，泛型变量T表示包含所有可能的类型。然而，尽管分别理解了这两件事，但我们无法把它们放在一起理解。在许多Rust新手头脑中，认为泛型是这样工作的：

|   |   |   |   |
|---|---|---|---|
|**Type Variable**|**T**|&T|&mut T|
|Examples|i32|&i32|&mut i32|

  

T包含所有拥有的类型；&T包含所有不可变借用的类型；&mut T包含所有可变借用类型。T， &T，和&mut是不相交的有限集合。很好，简单，干净，容易，直观。但这是完全错误的，下面是泛型在Rust中的实际工作方式：

|   |   |   |   |
|---|---|---|---|
|Type Variable|T|&T|&mut T|
|Examples|i32, &i32, &mut i32, &&i32, &mut &mut i32, ...|&i32, &&i32, &&mut i32, ...|&mut i32, &mut &mut i32, &mut &i32, ...|

T， &T和&mut T都是无穷集合，因为可以借用到一个无穷类型。T是&T和&mut T的超集，&T和&mut T是不相交的集合。以下是验证这些概念的几个例子：

```sh
trait Trait {}  
  
impl<T> Trait for T {}  
  
impl<T> Trait for &T {} // ❌  
  
impl<T> Trait for &mut T {} // ❌
```

上面的程序没有按预期编译：

```sh
error[E0119]: conflicting implementations of trait `Trait` for type `&_`:  
 --> src/lib.rs:5:1  
  |  
3 | impl<T> Trait for T {}  
  | ------------------- first implementation here  
4 |  
5 | impl<T> Trait for &T {}  
  | ^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`  
  
error[E0119]: conflicting implementations of trait `Trait` for type `&mut _`:  
 --> src/lib.rs:7:1  
  |  
3 | impl<T> Trait for T {}  
  | ------------------- first implementation here  
...  
7 | impl<T> Trait for &mut T {}  
  | ^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
```

编译器不允许我们为&T和&mut T定义Trait的实现，因为它会与Trait for T的实现冲突，Trait for T已经包含了所有的&T和&mut T。下面的程序按照预期编译，因为&T和&mut T是不相交的：

```sh
trait Trait {}  
  
impl<T> Trait for &T {} // ✅  
  
impl<T> Trait for &mut T {} // ✅
```

关键概念：

- T是&T和&mut T的超集
    
- &T和&mut T是不相交的集合
    

## 误解2：如果T: 'static '，那么T必须对整个程序有效

误解推论：

- T: 'static应该被读作“T有一个静态生命周期”。
    
- &'static T和T: 'static是一回事
    
- 如果T是静态的，那么T一定是不可变的
    
- 如果T: 'static，则T只能在编译时创建
    

大多数Rust初学者第一次接触“静态生命周期”的代码示例是这样的：

```rust
fn main() {  
    let str_literal: &'static str = "str literal";  
}
```

他们被告知“str_literal”被硬编码到编译后的二进制文件中，并在运行时加载到只读内存中，因此它对整个程序是不可变的且有效的，这就是使它成为“静态”的原因。围绕使用static关键字定义静态变量的规则进一步强化了这些概念。

```rust

static BYTES: [u8; 3] = [1, 2, 3];  
static mut MUT_BYTES: [u8; 3] = [1, 2, 3];  
  
fn main() {  
   MUT_BYTES[0] = 99; // ❌ - mutating static is unsafe  
  
    unsafe {  
        MUT_BYTES[0] = 99;  
        assert_eq!(99, MUT_BYTES[0]);  
    }  
}
```
关于静态变量：

- 它们只能在编译时创建
    
- 它们应该是不可变的，改变它们是不安全的
    
- 它们对整个项目都有效
    

具有静态生命周期的类型与受静态生命周期限制的类型是不同的。后者可以在运行时动态分配，可以安全自由地改变，可以被丢弃，并且可以存活任意的持续时间。

在这一点上，区分&'static T和T: 'static是很重要的。

&'static T是对某个T的不可变引用。只有当T本身是不可变的，并且在创建引用后不移动时，该T可以安全地无限期存在，直到程序结束。T不需要在编译时创建。可以在运行时生成随机动态分配的数据，并通过内存泄漏返回对它的静态引用。


```rust
// 在运行时生成随机的静态str引用  
fn rand_str_generator() -> &'static str {  
    let rand_string = rand::random::<u64>().to_string();  
    Box::leak(rand_string.into_boxed_str())  
}
```

T: 'static是可以安全地长期持有的T，直到程序结束。T: 'static包括所有&'static T，但它也包括所有拥有的类型，如String, Vec等。数据的所有者可以保证，只要所有者持有数据，数据就永远不会失效，因此所有者可以安全地无限期地持有数据，直到程序结束。T: 'static应该读成“T受“静态生命周期”的限制”而不是“T有一个静态生命周期”。

让我们看一下说明这些概念的程序：

```rust
fn main() {  
    let mut strings: Vec<String> = Vec::new();  
    for _ in 0..10 {  
        if rand::random() {  
            // 所有字符串都是在运行时随机生成和动态分配的  
            let string = rand::random::<u64>().to_string();  
            strings.push(string);  
        }  
    }  
  
    // 字符串是拥有的类型，所以它们被'static限定  
    for mut string in strings {  
        // 字符串是可变的  
        string.push_str("a mutation");  
        // 字符串是可drop的  
        drop_static(string); // ✅  
    }  
  
    // 在程序结束之前，所有字符串都已失效  
    println!("I am the end of the program");  
}  
  
fn drop_static<T: 'static>(t: T) {  
    std::mem::drop(t);  
}
```

关键概念：

- T: 'static应该读成“T受“静态生命周期”的限制
    
- T: 'static，则T可以是具有'静态生命周期'的借用类型或自有类型
    
- T: 'static包含了表示T的自有类型，意味着：
    

- 可以在运行时动态分配
    
- 不一定对整个程序都有效
    
- 可以安全自由地可变
    
- 可以在运行时动态删除
    
- 可以有不同长度的生命周期

## 误解3：&'a T和T: 'a是一样的

&'a T要求并意味着T: 'a，因为如果T本身对'a无效，那么对T的生命周期'a的引用就不能对'a有效。例如，Rust编译器永远不会允许构造&'static Ref<'a, T>类型，因为如果Ref只对'a有效，我们就不能对它进行静态引用。

T: 'a包含所有&'a T，但反过来则不成立。
```rust
fn main() {  
    let string = String::from("string");  
  
    t_bound(&string); // ✅  
    t_bound(Ref(&string)); // ✅  
    t_bound(&Ref(&string)); // ✅  
  
    t_ref(&string); // ✅  
    t_ref(Ref(&string)); // ❌ - expected ref, found struct  
    t_ref(&Ref(&string)); // ✅  
  
    t_bound(string); // ✅  
}  
  
// 只接受以'a为界的引用类型  
fn t_ref<'a, T: 'a>(t: &'a T) {}  
  
// 取任何以'a为界的类型  
fn t_bound<'a, T: 'a>(t: T) {}  
  
struct Ref<'a, T: 'a>(&'a T);
```
关键概念：

- T: 'a 比&'a T更通用，更灵活
    
- T: 'a 接受所有类型、包含引用的所有类型
    
- &'a T 仅接受引用
    
- 如果T: 'static 则也是T: 'a，因为'static >= 'a