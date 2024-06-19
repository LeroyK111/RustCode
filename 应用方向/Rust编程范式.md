# Rust中的编程范式：命令式和函数式编程范式

Rust是一种多范式编程语言，支持命令式、面向对象和函数式编程风格。风格的选择通常取决于开发人员的背景和他们要解决的具体问题。

随着Rust吸引了来自C++、Java、Python和Haskell等不同背景的开发人员，它已经形成了自己独特的风格和习惯用法。这种多样性是一种优势，但它也会导致在不同场景中使用哪种风格的不确定性。

Rust当然受到面向对象编程概念的影响，将它与其他面向对象语言区分开来的一个因素是它基于组合的特性，而不是基于继承的特性。它的Trait系统是面向对象设计的核心组成部分，这是C++和Java等语言所没有的概念。

类似地，Rust的设计也与函数式编程原则紧密结合：不变性、迭代器模式、代数数据类型和模式匹配等。

正如Rust采用某些面向对象的原则而不是纯粹的面向对象语言一样，它同样包含函数式编程概念而不是纯粹的函数式语言。

本文探讨了在Rust中选择不同范式时的个人决策过程，提供了一些关于在Rust中使用不同范式的指导，特别是对于从其他语言过渡过来的开发人员。

## 一个小例子

在Rust中使用简单的for循环并没有错。

```rust
let mut sum = 0;for i in 0..10 {    sum += i;}
```

即使在这样一个简短的示例中，我们也可以看到我们试图解决的问题与我们编写的代码之间的差异：我们不关心sum的中间值，只关心最后的结果。

将其与功能更强大的版本进行比较：

```rust
let sum: u32 = (0..10).sum();
```

在像这样的小示例中，这可能无关紧要，但是当我们开始使用嵌套循环时，我们看到在命令式方法中，更多的行专门用于临时记录，这会导致代码的复杂性增加(我们自己引入的不必要的复杂性)。复杂性，无论多小，都会耗费注意力。

## 一个更大的例子：嵌套循环

让我们看一个稍微大一点的例子，假设我们有一个编程语言的列表，包含它们支持的编程范式列表，以及每种语言的用户数量。我们的任务是找到支持函数式编程并且拥有最多用户的前五种语言。

```rust
let languages = vec![    Language::new("Rust", vec![Paradigm::Functional, Paradigm::ObjectOriented], 100_000),    Language::new("Go", vec![Paradigm::ObjectOriented], 200_000),    Language::new("Haskell", vec![Paradigm::Functional], 5_000),    Language::new("Java", vec![Paradigm::ObjectOriented], 1_000_000),    Language::new("C++", vec![Paradigm::ObjectOriented], 1_000_000),    Language::new("Python", vec![Paradigm::ObjectOriented, Paradigm::Functional], 1_000_000),];
```

下面是一个使用嵌套for循环的解决方案：

```rust
// 过滤语言，只保留支持函数式编程的语言let mut functional_languages = vec![];for language in languages {    if language.paradigms.contains(&Paradigm::Functional) {        functional_languages.push(language);    }}// 按用户数量降序对函数式语言进行排序for i in 1..functional_languages.len() {    let mut j = i;    while j > 0 && functional_languages[j].users > functional_languages[j - 1].users {        functional_languages.swap(j, j - 1);        j -= 1;    }}// 只保留前5种编程语言while functional_languages.len() > 5 {    functional_languages.pop();}
```

这是一个非常冗长、命令式的解决方案。我们就地改变矢量，并在此过程中破坏中间结果。虽然这是正确的，但我认为它不是最地道的Rust代码。

在实践中，你可能会从标准库中的适配器获取更多的帮助：

```rust
let mut top_languages = vec![];for language in languages {    if language.paradigms.contains(&Paradigm::Functional) {        top_languages.push(language);    }}// 把我们的语言按流行程度降序排序// 这一行在本质上已经有点函数式了top_languages.sort_by_key(|lang| std::cmp::Reverse(lang.users));top_languages.truncate(5);
```

我们不妨在过滤时更简洁一点：

```rust
let top_languages: Vec<Language> = languages    .iter()    // 只保留函数式语言    .filter(|language| language.paradigms.contains(&Paradigm::Functional))    // Sort our languages in descending order of popularity.    .sorted_by_key(|lang| Reverse(lang.users))    // 只保留前5种语言    .take(5)    // 将结果收集到一个新的向量中    .collect();
```

对整个列表进行排序(即使经过过滤)，只提取前5个元素似乎有些低效。这突出了Rust与C++相比的一个限制，C++在其标准库中提供了一个partial_sort函数。虽然Rust在std中没有等价的，但有第三方的crate。或者，也可以使用BinaryHeap。

这样做有几个原因：

- **可读性：步骤清晰
    
- **库支持：Rust标准库和外部crate为迭代器提供了许多有用的组合，它们可以很好地处理不可变数据结构。
    
- **效率：在底层，map和filter这样的方法创建了新的迭代器，这些迭代器操作前一个迭代器，并且不会产生任何分配。实际的计算只在最后一个迭代器被使用时执行，在本例中是由collect方法执行。collect方法进行一次分配，将结果存储在一个新向量中。我们的高级抽象不会产生运行时开销。
    
- **并行****性：**函数式方法适合并行计算。每个操作链都是独立的，允许它们在现代硬件上同时执行。


## Rust的面向对象编程范式和编程范式的最佳实践

与前面讨论的函数式和命令式示例相反，让我们引入一个新的结构体FileFilter，它封装了过滤文件和文件迭代的逻辑。

```rust
pub struct FileFilter {  
    predicates: Vec<Box<Predicate>>,  
    start: Option<PathBuf>,  
    stack: Vec<fs::ReadDir>,  
}
```
每个FileFilter对象都携带其状态：用于过滤的谓词集合、起始路径和用于迭代的目录栈。

Predicate是这样定义的:
```rust
type Predicate = dyn Fn(&Path) -> bool;
```
你可能会惊讶地发现这里有一个dyn。在Rust中，没有两个闭包具有相同的类型，即使它们完全相同！

为了在Vec集合中适应这一点，我们使用带有动态分派的trait对象。通过“装箱”这些闭包，我们创建了一个Box<Predicate类型(本质上是Box<dyn Fn(&Path) -> bool>)，这允许我们在同一个Vec中存储不同的Predicate闭包，尽管它们的类型不同。

在函数式编程中，我们利用迭代器和闭包的强大功能来过滤文件。在命令式风格中，我们直接使用循环和条件来操作向量。在面向对象风格中，结构体FileFilter抽象掉了这些细节。

看一下add_filter方法：

```rust
pub fn add_filter(mut self, predicate: impl Fn(&Path) -> bool + 'static) -> Self {  
    self.predicates.push(Box::new(predicate));  
    self  
}
```

这允许我们通过链接调用轻松地添加多个过滤器：

```rust
let filter = FileFilter::new()  
    .add_filter(|path| {  
        // 检查路径是否以“foo”开头  
        path.file_name()  
            .and_then(OsStr::to_str)  
            .map(|name| name.starts_with("foo"))  
            .unwrap_or(false)  
    })    
    .add_filter(|path| path.extension() == Some(OsStr::new("xml")));
```
真正展示Rust中面向对象方法的是FileFilter的Iterator trait的实现：
```rust
impl Iterator for FileFilter {  
    type Item = Result<PathBuf>;  
  
    fn next(&mut self) -> Option<Self::Item> {  
        todo!()  
    }  
}
```
这样，FileFilter就成为了一个构建块，它与Rust强大的迭代器生态系统完美地集成在一起，可以像其他任何迭代器一样在所有相同的地方使用，这种设计允许将复杂的迭代逻辑封装在对象中。

FileFilter示例说明了Rust中的OOP如何进行可靠的封装和模块化，我们将内容(谓词)与方式(迭代和过滤逻辑)分开。trait系统轻松地将自定义迭代器与生态系统的其余部分集成在一起，使用这些工具可以使代码更易于组合和重用。

编程范式最佳实践

在Rust中混合不同的编程风格不仅是可能的，而且是鼓励的！从Rust对其语言设计的主要理念中也可以看出这一点。C、Haskell、OCaml和Erlang等各种各样的影响塑造了Rust的设计。

一开始，Rust在本质上更偏向于函数式，但后来它演变成了一种更加平衡的语言，支持各种风格，问题是如何在不同的编程范例之间划清界限。

以下是一些最佳实践：

- 利用函数式风格进行数据转换，尤其是在函数和闭包等较小的范围内，map、filter或缩减等函数式方法可以使代码既简洁又清晰。
    
- 对于组织较大的应用程序，请考虑面向对象的风格。使用结构体或枚举可以封装相关的数据和函数，提供一个清晰的结构。
    
- 使用命令式风格进行粒度控制，在接近硬件的场景中，或者需要明确的分步执行时，命令式风格通常是必要的。它允许对操作进行精确控制，特别是对可变数据。这种风格在性能关键部分或与外部系统接口时特别有用，因为在这些地方需要精确的顺序。但是，始终要权衡其性能收益与潜在的可读性权衡。如果可能，将命令式代码封装在有限的范围内。
    
- 优先考虑可读性和可维护性，无论你选择哪种编程范式，都要始终编写简单且易于维护的代码。这不仅有利于未来的自己，而且也有利于在同一代码库上工作的同事。
    
- 避免过早优化，不要过早地以可读性为代价来优化性能，真正的瓶颈可能在其他地方。先测量，再优化。优雅的解决方案可以转化为快速的解决方案，但反过来并不总是正确的。

