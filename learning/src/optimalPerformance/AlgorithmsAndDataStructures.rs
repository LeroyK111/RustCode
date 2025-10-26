/*
优化算法和数据结构

编写高效Rust代码的另一个关键方面是优化程序中使用的算法和数据结构。
选择正确的数据结构数据结构的选择会对代码的性能产生巨大的影响。对于某些类型的问题，有些数据结构天生比其他数据结构更有效，这就是为什么选择一个非常适合的数据结构很重要。
例如，Rust中的标准HashMap数据结构被实现为哈希表，它提供恒定时间的平均情况查找、插入和删除。但是，具有类似功能的其他数据结构可能更适合特定问题，例如BTreeSet或BTreeMap，它们擅长于维护已排序的元素集合。这些数据结构为这些类型的操作提供对数的时间复杂度，在特定情况下，这可能比标准HashMap更有效。
一定要仔细考虑哪种数据结构最适合你的问题，因为选择正确的数据结构会对代码的性能产生巨大影响。
*/

/*
优化算法1，缓存结果一种常见的优化技术是缓存结果。如果一个函数可以用相同的输入调用多次，可以缓存结果并返回它，而不是每次都重新计算它。这对于减少需要执行昂贵的计算量尤其有用。
假设有一个执行数据库查询并返回结果的昂贵函数。如果使用相同的输入多次调用此函数，则可以简单地保存结果并返回它，而不是每次都重新计算结果，这可以显著提高代码的性能和效率。重要的是要记住，如果数据发生变化，则需要使缓存失效并重新计算结果。
*/

use std::collections::HashMap;

fn get_data_from_database(id: u32, cache: &mut HashMap<u32, String>) -> String {
    if let Some(data) = cache.get(&id) {
        return data.clone();
    }

    let data = perform_expensive_database_query(id);
    cache.insert(id, data.clone());
    data
}

fn perform_expensive_database_query(id: u32) -> String {
    // 模拟一个昂贵的数据库查询
    println!("Performing database query for ID {}", id);
    // ... 数据库访问和数据检索 ...
    let data = format!("Data for ID {}", id);
    data
}

fn main() {
    let mut cache: HashMap<u32, String> = HashMap::new();

    // 多次从数据库查询数据
    for _ in 0..5 {
        let id = 42;
        let data = get_data_from_database(id, &mut cache);
        println!("Data: {}", data);
    }
}
/*
运行上面的代码，我们可以看到数据库查询只执行一次，尽管我们使用相同的输入多次调用get_data_from_database函数。这是因为我们缓存结果并返回它，而不是每次都重新计算它，从而节省了执行不必要的昂贵查找的时间。
*/

/*
2，理解时间和空间的复杂度理解算法的时间复杂度对于编写高效代码至关重要。时间复杂度描述了算法的运行时间如何随着输入大小的增加而增长。通过选择具有更好时间复杂度的算法，可以显著提高代码的性能。例如，假设想对一个有10,000元素的列表进行排序，如果使用简单的冒泡排序算法，其平均时间复杂度为O(n2)，它比使用更有效的排序算法(如快速排序)花费的时间要长得多，快速排序的平均时间复杂度为O(nlog n)。
*/


/*
内存优化

除了优化算法和数据结构之外，内存优化是编写高效Rust代码的另一个重要方面。已知容量时指定容量在Rust中，可以将Vec类型用于动态数组。当向Vec添加元素时，它会自动管理底层缓冲区，并在必要时重新分配它。但是，这个重新分配过程涉及分配新内存、复制现有元素和释放旧缓冲区。为了避免不必要的重新分配，你可以使用with_capacity方法预先分配一个具有初始容量的Vec。通过提供匹配或超过预期元素数量的初始容量，可以减少或消除运行时期间的重新分配。这种类型的优化也适用于重新分配内存的数据结构，如string, hashmap等。
*/

let mut vec = Vec::with_capacity(100);

for i in 0..100 {
    vec.push(i);
}

/*
在这种情况下，Vec的初始化容量为100，因此它不需要在循环期间重新分配内存，因此运行速度更快。

避免不必要的克隆

在Rust中，克隆对象会创建对象的深层副本，这在内存和性能方面会很昂贵。因此，尽可能避免不必要的克隆是很重要的。避免不必要的克隆的一种方法是使用引用而不是拥有的值。引用允许借用一个值而不获得它的所有权，这意味着可以访问该值的只读版本而不需要克隆它：
*/

fn do_something_inefficiently() {
    fn process_vec(vec: Vec<i32>) -> i32 {
        vec.iter().sum()
    }

    let vec = (0..1000).collect::<Vec<u128>>()

    for _ in 0..10_000 {
        process_vec(vec.clone());
    }
}

fn do_something_with_speed() {
    fn process_vec(vec: &Vec<i32>) -> i32 {
        vec.iter().sum()
    }

    let vec = (0..1000).collect::<Vec<u128>>()

    for _ in 0..10_000 {
        process_vec(&vec);
    }
}

/*
通过将引用传递给vec，允许函数暂时借用它的值。这消除了不必要的内存分配和复制，提高了性能。clippy是一个查找不必要克隆的好工具，它是Rust的通用检查工具。它可以检测并警告你不必要的克隆，以及其他功能。请记住，clippy不会捕获所有内容，因此自己密切关注这些类型的优化非常重要。
*/

/*
对不同的数据类型使用枚举变量

如果需要一个可以包含不同类型元素的集合，请考虑使用枚举来表示不同的变体。这允许在同一个集合中存储不同类型的元素，而不会在填充或对齐上浪费内存：
*/

enum Element {
    Integer(i32),
    Float(f32),
}

fn main() {
    let mut elements: Vec<Element> = Vec::new();
    elements.push(Element::Integer(5));
    elements.push(Element::Float(3.14));

    for element in elements {
        match element {
            Element::Integer(i) => println!("Integer: {}", i),
            Element::Float(f) => println!("Float: {}", f),
        }
    }
}

/*
在本例中，Element枚举有两个变体：Integer和Float，它们各自可以保存不同的类型。在这里，我们创建了一个element向量，并将不同的类型推入其中，然后我们可以对其进行迭代操作。使用带有变量的enum可以让我们拥有一个可以存储不同类型值的集合，而不会浪费内存。这在处理异构数据结构或希望在单个变量中表示多种可能性时特别有用。
*/

/*
使用位标志(Bitflag)如果需要一个布尔值的集合，请考虑使用位标志而不是布尔值的向量，这可以显著减少内存使用并提高性能。位标志是一种数据结构，它将布尔值存储为位而不是字节，它允许在单个字节中存储多个布尔值。例如，让我们看看国际象棋的棋盘，棋盘可以用一个64位整数来表示棋盘的状态。整数中的每一位代表棋盘上的一个方格，值为1表示该方格已被占用，值为0表示该方格为空。象棋的起始位置可以表示为：1111111111111111000000000000000000000000000000001111111111111111

前8位代表第一行，后8位代表第二行，以此类推。如果方格被占用，每个位被设置为1，如果方格为空，则设置为0。通过位的组合，可以用几个64位整数表示整个棋盘的状态，可以一次对整个棋盘执行操作，这可以显著的提高性能。
*/


/*
使用Cow

在Rust中优化内存使用的另一种方法是使用Cow，即“写时克隆”类型。Cow类型拥有一个可以借用或拥有的值，这取决于是否需要修改它。当你有一个可能需要修改也可能不需要修改的值时，使用Cow可以帮助避免不必要的克隆和内存分配。Cow类型提供了两个变体：Borrowed和Owned，前者保存对原始值的引用，后者保存该值的已拥有副本。例如：
*/

use std::borrow::Cow;

fn process_string(s: Cow<&str>) {
    if s.len() > 10 {
        println!("Long string: {}", s);
    } else {
        println!("Short string: {}", s);
    }
}

fn main() {
    let short_string = "hello";
    let long_string = "this is a very long string";

    process_string(Cow::Borrowed(short_string));
    process_string(Cow::Borrowed(long_string));

    let cloned_long_string = long_string.to_owned();

    process_string(Cow::Owned(cloned_long_string));
}

/*
在这个例子中，process_string函数接受一个Cow<&str>作为参数，检查字符串的长度，并根据长度打印“长字符串”或“短字符串”。这里，我们创建了两个字符串变量:short_string和long_string，然后使用Cow:: borrow将它们传递给process_string函数。由于这些字符串不需要修改，所以它们是借来的，而不是克隆的。接下来，我们创建一个名为cloned_long_string的新变量，它是原始长字符串的自有副本。我们使用Cow::Owned变体将这个克隆的字符串传递给process_string函数，因为我们需要字符串的可变副本。通过使用适当的Cow变体，我们可以避免不必要的克隆和内存分配。如果一个值不需要修改，它可以被借用而不是被拥有。只有当一个值需要修改时，我们才创建一个拥有的副本。
*/

/*
避免为下一次迭代收集数据
在Rust中处理集合时，重要的是要考虑使用迭代器，而不是将集合收集到另一个数据结构中，然后再对其进行迭代。这有助于优化内存使用并提高性能。例如，假设我们有一组数字，我们想找出所有偶数的和：
*/


// 例如，假设我们有一组数字，我们想找出所有偶数的和：
fn sum_of_even_numbers(numbers: Vec<i32>) -> i32 {
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    even_numbers.iter().sum()
}
// 在这段代码中，我们首先通过从原始数字向量中过滤掉所有奇数来创建一个名为even_numbers的新Vec。然后使用iter()方法在even_numbers向量上创建一个迭代器，并使用sum()方法计算它们的和。然而，这种方法并不节省内存，因为它需要为迭代目的创建一个新的向量。相反，我们可以直接遍历过滤后的元素，而不必将它们收集到另一个数据结构中：

fn sum_of_even_numbers(numbers: Vec<i32>) -> i32 {
    numbers.into_iter().filter(|&x| x % 2 == 0).sum()
}
// 在这段更新后的代码中，我们使用into_iter()方法直接遍历过滤后的元素，并使用sum()方法计算它们的和。这避免了创建新的向量并提高了内存效率。


/*
内联
内联是一种编译器优化技术，它用函数的实际主体替换函数调用，这可以通过减少函数调用的开销来进一步优化以提高性能。在Rust中，你可以使用#[inline]属性向编译器建议一个函数应该内联。编译器可能会根据代码大小和性能影响等各种因素选择是否接受这个建议。例如：
*/

#[inline]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(1, 2);
    println!("Result: {}", result);
}
// 在这段代码中，我们定义了一个简单的add函数，它接受两个整数并返回它们的和。我们用#[inline]注释它来建议内联。当在主函数中调用这个函数时，编译器可能会选择内联它，而不是生成一个单独的调用指令。内联可以通过消除函数调用的开销来提高性能，因为它提供了更好的优化机会，比如常量传播和循环展开。但是，如果过度使用，它也会增加代码大小，因此应该谨慎使用。


/*
Rust构建配置Rust提供了许多构建配置选项，允许控制Rust编译器如何生成机器代码并优化程序的各个方面。
在编译Rust代码时，使用--release标志来启用优化是很重要的。默认情况下，Rust以Debug模式构建，其中包括用于调试目的的额外检查和信息，从而牺牲了性能。要使用release构建代码，请使用以下命令：
*/



/*
cargo build --release
--release标志告诉Cargo启用内联函数调用、删除不必要的检查和优化数据结构等。这可以显著提高性能。在底层，这与使用RUSTFLAGS -C opt-level=3相同。
*/

/*
链接时优化链接时优化是一种技术，它允许编译器在链接阶段跨多个转化单元执行优化，从而实现更积极的优化和更好的运行时性能。要在Rust代码中启用LTO，可以在Cargo.toml文件中使用LTO选项：
[profile.release]
lto = fat
这将支持跨项目中所有代码单元的链接时优化，这可以带来更好的性能，但可能会增加二进制文件的大小。请记住，启用LTO可能会增加构建时间和内存使用。
*/

/*
编码单元编码单元是Rust编译器用来并行生成代码的编译单元。默认情况下，Rust编译器在每个CPU核心上使用一个编码单元，这允许更快的编译时间。然而，使用多个编码单元可以让编译器独立优化每个单元来提高运行时性能。可以通过在Cargo.toml文件中设置codegen-units选项来控制Rust编译器使用的编码单元的数量。
[profile.release]
codegen-units = 1
*/

/*
使用代替的内存分配器默认情况下，Rust使用系统的标准库的内存分配器来管理内存。但是，在某些情况下，希望使用能提供更好性能或特定功能的内存分配器。要在Rust代码中使用另一个分配器，可以在Cargo.toml文件中指定它。
[dependencies]
jemallocator = "0.3"
请注意，使用另一种分配器有可能并不能带来更好的性能，对代码进行基准测试以确定是否值得使用另一种分配器是很重要的。
*/

/*
编译标志Rust提供了几个编译器标志，允许控制代码生成和优化的各个方面。可以使用RUSTFLAGS环境变量或直接在Cargo.toml文件中设置这些标志。以下是一些常用的编译器标志：-C target-cpu：指定目标CPU架构。这允许编译器生成针对特定CPU优化的机器码。例如，将target-cpu设置为native将告诉编译器为这台机器的CPU寻找优化。-C debuginfo：控制是否在生成的二进制文件中包含调试信息。禁用调试信息可以减少二进制文件的大小，但会使调试变得更加困难。-C panic=abort：通过中止而不是展开来改变恐慌的处理方式，这可以提高性能，但代价是无法使用catch_unwind捕获恐慌。
*/