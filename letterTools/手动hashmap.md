# Rust挑战 - 动手实现HashMap 1

让我们在Rust中实现一个哈希表数据结构，由于哈希表的效率和通用性，它在数据结构中非常重要。通过从头开始实现它，我们可以深入了解所涉及的底层算法和数据结构。同时还还会提高我们的Rust技能。

说到算法，我们将实现线性探测开放寻址哈希表。

我们将采用自顶向下的方法，从较高级别的抽象开始，然后逐步向下到较低级别的抽象和实现。让我们从最顶层的抽象开始：API，我们不支持Rust标准库std::collections::HashMap的整个API，只是实现其核心功能。

```rust
use std::borrow::Borrow;
use std::hash::Hash;

pub struct HashMap<Key, Val> {
    // todo
}

impl<Key: Eq + Hash, Val> HashMap<Key, Val> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn insert(&mut self, key: Key, val: Val) -> Option<Val> {
        todo!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Val>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        todo!()
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Val>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        todo!()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<Val>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        todo!()
    }
}
```

这些方法具有与标准库中HashMap完全相同的签名。顺便说一下，如果你不熟悉API中出现的Borrow特性，请参考《Rust技巧：Borrow Trait》这篇文章。

哈希表基本上是一个项的数组，其索引是键的哈希值对数组大小的模。我们自然会对这个数组使用Vec，但是这个类型T是什么呢？我们叫它Entry，想想Entry应该包含什么。

我们知道Entry可以是空的，也可以是被占用的。当它被占用时，它应该同时包含Key和Val。为了支持remove()方法，我们需要第三个状态：Tombstone。此状态表示Entry曾经被占用但当前为空。稍后我们将了解为什么需要这样的区分，先看看我们的Entry结构体：

```rust
enum Entry<Key, Val> {
    Vacant,
    Tombstone,
    Occupied { key: Key, val: Val },
}
```

哈希表结构体需要两个usize字段——一个用于跟踪已占用的Entry，另一个用于跟踪空Entry。第一个字段是len()方法的返回值，第二个字段让我们决定何时调整数组的大小。这样，我们就可以在骨架中添加一些实现细节。代码修改如下：

```rust
pub struct HashMap<Key, Val> {
    xs: Vec<Entry<Key, Val>>,
    n_occupied: usize,
    n_vacant: usize,
}

impl<Key: Eq + Hash, Val> HashMap<Key, Val> {
    pub fn new() -> Self {
        Self {
            xs: Vec::new(),
            n_occupied: 0,
            n_vacant: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.n_occupied
    }
    ......
}
```

我们需要一个方法来计算键的哈希值，并对数组大小取模来获得索引。增加如下方法：

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl<Key: Eq + Hash, Val> HashMap<Key, Val> {
    ......

    fn get_index<Q>(&self, key: &Q) -> usize
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.xs.len()
    }
}
```

现在，让我们实现get()方法。我们所需要做的就是遍历从index开始的entry并比较键，直到找到匹配项，或者直到遇到空entry。在搜索过程中，我们简单地忽略并跳过tombstone状态的entry。


```rust
impl<Key: Eq + Hash, Val> HashMap<Key, Val> {
    ......

    pub fn get<Q>(&self, key: &Q) -> Option<&Val>
    where
        Key: Borrow<Q>,
        Q: Eq + Hash,
    {
        if self.len() == 0 {
            return None;
        }
        let mut idx = self.get_index(key);
        loop {
            match &self.xs[idx] {
                Entry::Vacant => {
                    break None;
                }
                Entry::Occupied { key: k, val } if k.borrow() == key => {
                    break Some(val);
                }
                _ => {
                    idx = (idx + 1) % self.xs.len();
                }
            }
        }
    }

    ......
}
```

# Rust挑战 - 动手实现HashMap 2

解决方案是通过迭代器循环遍历entry，而不是老式的索引计数。由于我们需要从给定的索引开始，循环遍历整个数组，以index-1结束，这本身有点棘手，但可以使用Iterator::split_at_mut()方法完成。这样，我们就可以最终实现get_mut()方法了。

```rust
impl<Key: Eq + Hash, Val> HashMap<Key, Val> {  
    ......  
  
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Val>  
    where  
        Key: Borrow<Q>,  
        Q: Eq + Hash,  
    {  
        if self.len() == 0 {  
            return None;  
        }  
        let idx: usize = self.get_index(key);  
        for entry in self.iter_mut_starting_at(idx) {  
            match entry {  
                Entry::Vacant => {  
                    return None;  
                }  
                Entry::Occupied { key: k, val } if (k as &Key).borrow() == key => {  
                    return Some(val);  
                }  
                _ => {}  
            }  
        }  
        panic!("fatal: unreachable");  
    }  
  
    fn iter_mut_starting_at(&mut self, idx: usize) -> impl Iterator<Item = &mut Entry<Key, Val>> {  
        let (s1, s2) = self.xs.split_at_mut(idx);  
        s2.iter_mut().chain(s1.iter_mut())  
    }  
  
    ......  
}
```

现在只剩下两个方法：insert()和remove()。现在是讨论Entry枚举Tombstone变量有什么作用的时候了。

我们的哈希冲突解决方案是从索引开始，探测被占用Entry的链，直到找到匹配的键，或者直到找到一个空的键，这标志着哈希冲突键的结束。

如果我们在链的中间找到匹配的键，并通过将其标记为Vacant来删除它，那么我们将链分成了两部分。下次我们搜索相同的链时，我们将无法搜索链的后半部分，因为我们将到达中间的空Entry。

这就是为什么我们不能简单地删除Entry并将其标记为Vacant。Tombstone变量让我们知道那里什么都没有，但我们仍然需要继续探索。

有了它，我们就可以实现remove()方法——如果我们找到一个具有匹配键的Entry，我们将其标记为Tomtsbone并递减计数器。否则，它是一个no-op。这听起来很简单，但在Rust中实现起来有点棘手。让我们首先实现一个从Entry获取值的辅助方法。

```rust
enum Entry<Key, Val> {  
    Vacant,  
    Tombstone,  
    Occupied { key: Key, val: Val },  
}  
  
use std::mem::swap;  
  
impl<Key, Val> Entry<Key, Val> {  
    fn take(&mut self) -> Option<Val> {  
        match self {  
            Self::Occupied { key, val } => {  
                let mut occupied = Self::Tombstone;  
                swap(self, &mut occupied);  
                if let Self::Occupied { key, val } = occupied {  
                    Some(val)  
                } else {  
                    panic!("fatal: unreachable");  
                }  
            }  
            _ => None,  
        }  
    }  
}
```

```rust
impl<Key: Eq + Hash, Val> HashMap<Key, Val> {  
    ......  
  
    pub fn remove<Q>(&mut self, key: &Q) -> Option<Val>  
    where  
        Key: Borrow<Q>,  
        Q: Eq + Hash,  
    {  
        if self.len() == 0 {  
            return None;  
        }  
        let idx = self.get_index(key);  
        let mut result = None;  
        for entry in self.iter_mut_starting_at(idx) {  
            match entry {  
                Entry::Occupied { key: k, .. } if (k as &Key).borrow() == key => {  
                    result = entry.take();  
                    break;  
                }  
                Entry::Vacant => {  
                    result = None;  
                    break;  
                }  
                _ => {}  
            }  
        }  
        result.map(|val| {  
            self.n_occupied -= 1;  
            val  
        })  
    }  
  
    ......  
}
```

现在只剩下最后一个方法：insert()。此方法首先检查负载因子并在必要时调整数组的大小。完成所有这些之后，它将把键、值对插入到表中。让我们创建一个辅助方法insert_helper()来执行插入部分，不需要检查负载因子以调整大小。我们另外分别定义计算负载因子和调整大小的方法。

```rust
impl<Key: Eq + Hash, Val> HashMap<Key, Val> {  
    ......  
  
    pub fn insert(&mut self, key: Key, val: Val) -> Option<Val> {  
        if self.load_factor() >= 0.75 {  
            self.resize();  
        }  
  
        self.insert_helper(key, val)  
    }  
  
    fn load_factor(&self) ->  f64 {  
        todo!()  
    }  
  
    fn resize(&mut self) {  
        todo!()  
    }  
  
    fn insert_helper(&mut self, key: Key, val: Val) -> Option<Val> {  
        todo!()  
    }  
  
    ......  
}
```

load_factor()方法很简单，但是存在一种极端情况—我们使用空数组初始化哈希表，因此我们需要显式地处理这种情况。至于resize()方法，我们希望将数组的大小增加一倍，除非当前的大小太小。最简单的实现是创建一个哈希表的新实例，简单地插入当前哈希表中的每个元素，最后交换这两个哈希表。

```rust
fn load_factor(&self) ->  f64 {  
    if self.xs.is_empty() {  
        1.0  
    } else {  
        1.0 - self.n_vacant as f64 / self.xs.len() as f64  
    }  
}  
  
fn resize(&mut self) {  
    let new_size = std::cmp::max(64, self.xs.len() * 2);  
    let mut new_table = Self {  
        xs: (0..new_size).map(|_| Entry::Vacant).collect(),  
        n_occupied: 0,  
        n_vacant: new_size,  
    };  
    for entry in std::mem::take(&mut self.xs) {  
        if let Entry::Occupied { key, val } = entry {  
            new_table.insert_helper(key, val);  
        }  
    }  
  
    swap(self, &mut new_table);  
}
```

好了，现在只剩下insert_helper()方法。从概念上讲，这并不太难。我们探测Entry并查找匹配的键。如果找到，则覆盖该值。如果没有，插入到空Entry，不要忘记相应地更新计数器。

```rust
impl<Key, Val> Entry<Key, Val> {  
    ......  
  
    fn replace(&mut self, mut x: Val) -> Option<Val> {  
        match self {  
            Self::Occupied { key, val } => {  
                swap(&mut x, val);  
                Some(x)  
            }  
            _ => None,  
        }  
    }  
}
```
```rust
impl<Key: Eq + Hash, Val> HashMap<Key, Val> {  
    ......  
    fn insert_helper(&mut self, key: Key, val: Val) -> Option<Val> {  
        let idx = self.get_index(&key);  
        let mut result = None;  
        for entry in self.iter_mut_starting_at(idx) {  
            match entry {  
                Entry::Occupied { key: k, .. } if (k as &Key).borrow() == &key => {  
                    result = entry.replace(val);  
                    break;  
                }  
                Entry::Vacant => {  
                    *entry = Entry::Occupied { key, val };  
                    break;  
                }  
                _ => {}  
            }  
        }  
        if result.is_none() {  
            self.n_occupied += 1;  
            self.n_vacant -= 1;  
        }  
        result  
    }  
    ......  
}
```

现在我们已经完成了Rust中HashMap的实现。

# Rust挑战 - 动手实现HashMap 3 性能优化、单元测试和基准测试

对于测试，将以随机顺序使用随机的键和值调用insert()、get()、get_mut()、remove()和len()五个方法数百万次。对于每次迭代，都会将结果与从标准库获得的结果进行比较。该测试确保两个实现具有完全相同的行为。

单元测试

在Cargo.toml文件中加入rand库：
```toml
[dependencies]  
rand = "0.8.5"
```

```rust
#[cfg(test)]  
mod tests {  
    use super::*;  
    use rand::distributions::*;  
  
    #[test]  
    fn run_test() {  
        let mut map1 = HashMap::new(); // our implementation  
        let mut map2 = std::collections::HashMap::new(); // standard library implementation  
  
        let key_gen = Uniform::from(0..1000); // random key, value  
        let op_gen = Uniform::from(0..5); // operation selection  
        let mut rng = rand::thread_rng();  
        for _ in 0..10000000 {  
            let val = key_gen.sample(&mut rng);  
            let key = val;  
            match op_gen.sample(&mut rng) {  
                0 => {  
                    // insert  
                    assert_eq!(map1.insert(key, val), map2.insert(key, val));  
                }  
                1 => {  
                    // get mut  
                    assert_eq!(  
                        map1.get_mut(&key).map(|x| {  
                            *x += 1;  
                            x  
                        }),  
                        map2.get_mut(&key).map(|x| {  
                            *x += 1;  
                            x  
                        })  
                    );  
                }  
                2 => {  
                    assert_eq!(map1.get(&key), map2.get(&key));  
                }  
                3 => {  
                    assert_eq!(map1.remove(&key), map2.remove(&key));  
                }  
                _ => {  
                    assert_eq!(map1.len(), map2.len());  
                }  
            }  
        }  
    }  
}
```
在Cargo.toml文件中写入如下内容：
```toml
[dev-dependencies]  
criterion = "0.3"  
  
[[bench]]  
name = "hashmap_benchmark"  
harness = false
```
在项目根目录下创建 benches/hashmap_benchmark.rs文件。

代码如下：

```rust
use criterion::{Criterion, criterion_group, criterion_main};  
use rand::{distributions::Uniform, prelude::Distribution};  
  
fn run_bench_hashmap() {  
    let mut map = hash_table::HashMap::new();  
  
    let key_gen = Uniform::from(0..1000);  
    let op_gen = Uniform::from(0..4);  
    let mut rng = rand::thread_rng();  
    for _ in 0..10000000 {  
        let val = key_gen.sample(&mut rng);  
        let key = val;  
        match op_gen.sample(&mut rng) {  
            0 => {  
                // insert  
                map.insert(key, val);  
            }  
            1 => {  
                // get mut  
                map.get_mut(&key).map(|x| {  
                    *x += 1;  
                    x  
                });  
            }  
            2 => {  
                // get  
                map.get(&key);  
            }  
            3 => {  
                // remove  
                map.remove(&key);  
            }  
            _ => {}  
        }  
    }  
}  
  
fn run_bench_std_hashmap() {  
    let mut map = std::collections::HashMap::new();  
  
    let key_gen = Uniform::from(0..1000);  
    let op_gen = Uniform::from(0..4);  
    let mut rng = rand::thread_rng();  
    for _ in 0..10000000 {  
        let val = key_gen.sample(&mut rng);  
        let key = val;  
        match op_gen.sample(&mut rng) {  
            0 => {  
                // insert  
                map.insert(key, val);  
            }  
            1 => {  
                // get mut  
                map.get_mut(&key).map(|x| {  
                    *x += 1;  
                    x  
                });  
            }  
            2 => {  
                // get  
                map.get(&key);  
            }  
            3 => {  
                // remove  
                map.remove(&key);  
            }  
            _ => {}  
        }  
    }  
}  
  
fn criterion_benchmark(c: &mut Criterion) {  
    c.bench_function("hash map", |b| b.iter(|| run_bench_hashmap));  
    c.bench_function("std hash map", |b| b.iter(|| run_bench_std_hashmap));  
}  
  
criterion_group!(benches, criterion_benchmark);  
criterion_main!(benches);
```

与test函数类似，在给定的哈希表上使用随机键、值重复运行。
![](../learning/src/objInfo/assets/Pasted%20image%2020240427205217.png)

差别是巨大的！运行同一基准测试所需的运行时差异和#instructions相差20倍。发生了什么事？

在对程序进行测试和基准测试时，我们应该考虑另一个指标：内存消耗。

我们的resize()方法非常低效地管理内存，这是主要的性能瓶颈。实际上，我们无条件地将数组大小加倍，即使几乎都是没有被占用的Entry。更合适的方法是检查占用的因子，并决定是保持数组大小还是将其加倍。

代码修改如下：

```rust
impl<Key: Eq + Hash, Val> HashMap<Key, Val> {  
   fn occupied_factor(&self) -> f64 {  
        if self.xs.is_empty() {  
            1.0  
        } else {  
            self.n_occupied as f64 / self.xs.len() as f64  
        }  
    }  
  
    fn resize(&mut self) {  
        // let new_size = std::cmp::max(64, self.xs.len() * 2);  
        let resize_factor = if self.occupied_factor() > 0.75 { 2 } else { 1 };  
        let new_size = std::cmp::max(64, self.xs.len() * resize_factor);  
        let mut new_table = Self {  
            xs: (0..new_size).map(|_| Entry::Vacant).collect(),  
            n_occupied: 0,  
            n_vacant: new_size,  
        };  
        for entry in std::mem::take(&mut self.xs) {  
            if let Entry::Occupied { key, val } = entry {  
                new_table.insert_helper(key, val);  
            }  
        }  
  
        swap(self, &mut new_table);  
    }  
}
```
通过这个简单的更改，我们的实现改进了10倍以上，并且我们的实现与标准库相当！
![](../learning/src/objInfo/assets/Pasted%20image%2020240427205246.png)