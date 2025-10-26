# 延迟初始化

在Rust的旧版本中，其标准库不支持这种延迟初始化。
在生态系统中有几个流行的crate通常用于此功能，例如lazy_static和once_cell。从Rust 1.80开始，这些crate提供的许多功能现在可以在标准库中使用，并且可以用来代替这两个crate。在这篇文章中，我们将介绍什么是延迟初始化模式，lazy_static和once_cell如何提供延迟初始化的功能，如何使用标准库进行延迟初始化，标准库与lazy_static和once_cell之间的比较。

考虑一个示例，我们有一个不经常使用的API接口，需要从磁盘读取和解析一个大文件。
我们可以在应用程序开始时执行读取和解析，但是，这可能会阻止服务器在解析完成之前提供服务。
还有一种情况是，这个特定的API接口根本没有被调用，因此用于加载文件的资源是没有用的。

另一个例子是，如果应用程序使用一些内存DB，如sqlite或redis。但是，实际上并非应用程序的所有调用都需要数据库。在这种情况下，将DB加载到内存中并每次维护连接开销是没必要的。我们可以将这些资源的初始化推迟到需要的时候，在第一次使用时初始化它们，并保留它们以供以后使用，这种模式被称为延迟初始化模式。
然而，这在Rust中出现了一个小问题，我们必须将延迟初始化的资源作为参数传递给每个函数，或者将其设置为静态全局的，并在运行时使用unsafe的Rust代码对其进行初始化。
为了避免这种情况，像lazy_static或once_cell这样的crate为不安全的操作提供了安全的封装，我们可以使用它们在代码中安全地使用延迟初始化的值。

## lazy_static和once_cell如何提供延迟初始化

lazy_static提供了一个宏来编写静态变量的初始化代码，并且在运行时第一次使用时初始化变量。一般语法是：
```rust
use lazy_static::lazy_static;
lazy_static!{
  static ref VAR : TYPE = {initialization code}
}
```
例如，将日志级别设置为静态变量，如下所示：
```rust
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG_LEVEL: String = get_log_level();
}

fn get_log_level() -> String {
    match std::env::var("LOG_LEVEL") {
        Ok(s) => s,
        Err(_) => "WARN".to_string(),
    }
}

fn main() {
    println!("{}", *LOG_LEVEL);
}
```

lazy_static!宏定义在运行时使用get_log_level函数来设置日志级别。虽然这很简单，但它也有一些自己的问题。我们必须使用静态ref，这不是一个有效的Rust语法，我们需要取消对LOG_LEVEL的引用，以便在println语句中使用。我们可以使用once_cellcrate做同样的事情：

```rust
use once_cell::sync::OnceCell;

static LOG_LEVEL: OnceCell<String> = OnceCell::new();

fn get_log_level() -> String {
    match std::env::var("LOG_LEVEL") {
        Ok(s) => s,
        Err(_) => "WARN".to_string(),
    }
}

fn main() {
    let log_level = LOG_LEVEL.get_or_init(get_log_level);
    println!("{}", log_level);
}
```

在这里，我们没有在声明中指定代码，而是在需要获取值时使用了get_or_init方法。如果值未初始化，则使用给定函数初始化该值，否则将返回现有值。因为我们直接获取值，所以不需要任何额外的解引用操作。虽然这两种方法各有优缺点，但是需要在依赖项中再添加一个外部crate。Rust 1.80后的标准库提供了延迟初始化的类型，我们就可以直接使用这些类型，从而减少依赖项的数量。

## 使用标准库进行延迟初始化

在Rust 1.80后，类似lazy_static和once_cell的延迟初始化类型已经在Rust标准库中稳定下来了。我们可以使用它们代替任何外部crate来实现类似的功能。标准库中的OnceLock类型可以类似于once_cellcrate的OnceCell类型使用：

```rust
use std::sync::OnceLock;

static LOG_LEVEL: OnceLock<String> = OnceLock::new();

fn get_log_level() -> String {
    match std::env::var("LOG_LEVEL") {
        Ok(s) => s,
        Err(_) => "WARN".to_string(),
    }
}

fn main() {
    let log_level = LOG_LEVEL.get_or_init(get_log_level);
    println!("{}", log_level);
}
```

与once_cell的例子相比，我们用OnceLock代替了OnceCell，但其余的代码仍然是相同的。OnceLock类型还公开了一个名为get_or_init的方法，该方法提供了与OnceCell的get_or_init相同的功能。与lazy_static相比，我们可以使用LazyLock类型在声明级别指定初始化函数，而不必使用宏：

```rust
use std::sync::LazyLock;

static LOG_LEVEL: LazyLock<String> = LazyLock::new(get_log_level);

fn get_log_level() -> String {
    match std::env::var("LOG_LEVEL") {
        Ok(s) => s,
        Err(_) => "WARN".to_string(),
    }
}

fn main() {
    println!("{}", *LOG_LEVEL);
}
```
这里我们传递一个初始化函数给LazyLock的new方法，当变量的值第一次被访问时，类型内部调用这个函数来初始化这个值。

## 标准库与lazy_static和once_cell之间的比较

与lazy_static和once_cell这两种crate相比，标准库提供的延迟初始化类型的一大优点是不需要任何额外的依赖项。尽管这两个crate本身只有几个依赖项，但这仍然意味着你的项目将有更多的依赖项，并且需要更多的编译时间。标准库提供的延迟初始化类型的另一个优点是它们是由官方rust标准库团队直接开发和维护的。