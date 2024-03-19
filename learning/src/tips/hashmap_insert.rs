/*
在对代码进行优化时，如果想在hashmap、hashset或任何类似的结构中插入元素，需要注意一个事项。

你可能想在代码中做以下一些事情：

在对代码进行优化时，如果想在hashmap、hashset或任何类似的结构中插入元素，需要注意一个事项。

你可能想在代码中做以下一些事情：
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(3);

    assert_eq!(map["poneyland"], 3);
}
但是如果在插入时需要做更多的工作呢？可以这样做：
map.entry("poneyland").or_insert(do_something);
这里的问题是，如果键“poneyland”已经存在于hashmap中，do_something仍然会被调用。如果do_something消耗时间或CPU，那么代码的性能将会下降。

在这里，你应该做的是使用or_insert_with方法，并像这样传递一个闭包：
map.entry("poneyland").or_insert_with(|| {
    do_something()
};
这里编写了一些快速测试的代码，以便你可以自己进行验证：
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::{thread, time};

    fn compute_value_to_insert() -> usize {
        println!("My insertion function starts.");
        thread::sleep(time::Duration::from_secs(2));
        println!("My insertion function finished.");
        0
    }

    #[test]
    fn or_insert() {
        let mut h: HashMap<usize, usize> = HashMap::new();
        h.insert(0, 0);
        h.entry(0).or_insert(compute_value_to_insert());
    }

    #[test]
    fn or_insert_with() {
        let mut h: HashMap<usize, usize> = HashMap::new();
        h.insert(0, 0);
        h.entry(0).or_insert_with(|| compute_value_to_insert());
    }
}

当执行or_insert测试时，结果如下：
running 1 test
test tests::or_insert ... ok

test result: ok. 1 passed; 0 failed; finished in 2.00s

当执行or_insert_with测试时，结果如下：
running 1 test
test tests::or_insert_with ... ok

test result: ok. 1 passed; 0 failed; finished in 0.00s

从测试结果可以看出，当使用or_insert方法时，执行完成时间是2秒；当使用or_insert_with方法时，执行完成时间是近乎0秒。性能提升还是很明显的。
*/


