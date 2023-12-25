/*
! 十种常用的迭代器
*/

fn main() {
    /*
    *1. Map

    map适配器是一种通用工具，可以使用闭包或函数转换迭代器的每个元素。它对集合中的每个元素应用自定义的逻辑，用转换后的元素生成一个新的迭代器。

    通过利用map适配器，可以轻松地执行诸如数据处理、提取等操作。
        */
    let numbers = [1, 2, 3, 4, 5];
    let squared = numbers.iter().map(|it| it * it).collect::<Vec<_>>();
    println!("平方: {squared:?}");
    // 平方: [1, 4, 9, 16, 25]

    /*
    *2. Filter

    filter适配器能够根据给定条件有选择地从迭代器中选择元素。它对每个元素应用筛选谓词，并仅返回满足指定条件的元素。

    通过使用filter适配器，可以有效地提取满足特定需求的数据，从而增强代码的清晰度和效率。
        */

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_numbers = numbers.iter().filter(|it| *it % 2 == 0).collect::<Vec<_>>();
    println!("偶数集合: {even_numbers:?}");
    // 偶数集合: [2, 4, 6, 8, 10]

    /*
        *Fold
    fold适配器，也称为简化迭代器或累积迭代器，将迭代器中的元素聚合为单个值。它从一个初始值开始，并对每个元素应用一个fold函数，迭代地累积结果。

    在处理计算总和、查找最大值或最小值，甚至连接字符串等任务时，这个迭代器是相当有用的。

    例如，要计算迭代器中所有数字的乘积，你可以编写以下代码：
        */

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let product = numbers.iter().fold(1, |acc, it| acc * it);
    println!("Product: {product}");
    // Product: 3628800

    /*
        *Zip

    zip适配器将多个迭代器组合成一个迭代器，将相应的元素配对在一起。当任何一个输入迭代器耗尽时，它就会停止。

    使用zip适配器，可以方便地同时遍历多个集合，执行需要来自多个数据源数据的操作。
        */

    let names = ["Alice", "Bob", "Charlie"];
    let ages = [25, 30, 28];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("Name: {name}, age: {age}");
    }
    // Name: Alice, age: 25
    // Name: Bob, age: 30
    // Name: Charlie, age: 28

    /*
        *Chunks

    chunks适配器将迭代器划分为固定大小的块，并将它们作为单独的较小的迭代器返回。这在需要批量处理数据或对集合的子集执行操作时特别有用。

    chunks适配器通过将大型数据集分解成更易于管理的部分，简化了对它们的处理。
        */

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for group in numbers.chunks(3) {
        println!("Group: {group:?}");
    }
    // Group: [1, 2, 3]
    // Group: [4, 5, 6]
    // Group: [7, 8, 9]

    /*
    *Chain
        chain适配器连接两个或多个迭代器，创建一个按顺序遍历每个迭代器元素的单个迭代器。它组合不同的集合或从多个源生成复合迭代器。

    通过利用chain迭代器，可以无缝地处理来自不同迭代器的数据，就好像它们是单个集合一样。
        */

    let numbers_2021 = [1, 2, 3];
    let numbers_2022 = [4, 5, 6];
    let numbers_2023 = [7, 8, 9];

    for number in numbers_2021
        .iter()
        .chain(numbers_2022.iter())
        .chain(numbers_2023.iter())
    {
        println!("Number: {number}");
    }
    // Number: 1
    // Number: 2
    // Number: 3
    // Number: 4
    // Number: 5
    // Number: 6
    // Number: 7
    // Number: 8
    // Number: 9

    /*
        *All

    all适配器检查迭代器的所有元素是否满足给定条件。如果条件对所有元素都成立，则返回true，否则返回false。

    当需要验证集合中每个元素的属性，确保它们在进行进一步操作之前满足某些标准时，此适配器很有帮助。
        */

    let numbers = [2, 4, 6, 8, 10];

    if numbers.iter().all(|it| *it % 2 == 0) {
        println!("都是偶数");
    } else {
        println!("都不是偶数");
    }
    // 都是偶数

    /*
        *Any

    any适配器确定迭代器中的任何元素是否满足指定条件。一旦遇到第一个符合条件的项，它就返回true，如果没有元素满足条件则返回false。

    通过使用any适配器，可以有效地检查集合中是否至少有一项满足所需的条件。
        */

    let numbers = [10, 30, 50, 25, 150, 50, 30];

    if numbers.iter().any(|it| *it > 100) {
        println!("有值大于 100");
    } else {
        println!("没有值大于 100");
    }
    // 有值大于 100

    /*
        * Windows
        windows适配器在迭代器上生成滑动窗口，生成指定大小的连续元素组。它处理集合的重叠或连续子集，执行依赖于相邻元素的计算。

    windows适配器是一个强大的工具，用于分析数据中的模式或依赖关系。
        */

    let numbers = [1, 2, 3, 4, 5, 8, 9, 10, 15];

    if numbers.windows(2).all(|group| group[0] < group[1]) {
        println!("按升序排序");
    } else {
        println!("不是按升序排序");
    }
    // 按升序排序

    /*
        *Cycle

    cycle适配器无限地重复迭代器中的元素，形成一个无限循环。一旦迭代器到达终点，它就会重新启动迭代器，重复迭代集合。

    当需要以循环方式遍历一组元素或对连续的数据流执行操作时，循环迭代器非常方便。
        */

    let nice_numbers = [5, 10, 15];
    let more_nice_numbers = nice_numbers.iter().cycle().take(10).collect::<Vec<_>>();
    println!("numbers: {more_nice_numbers:?}");
    // numbers: [5, 10, 15, 5, 10, 15, 5, 10, 15, 5]
}
