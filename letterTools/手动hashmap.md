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

哈希表基本上是一个项的数组，其索引是键的哈希值对数组大小的模。我们自然会对这个数组使用Vec<T>，但是这个类型T是什么呢？我们叫它Entry，想想Entry应该包含什么。

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

目前看，还不错。我们可以对get_mut()方法做同样的事情吗？如果简单地将不可变变量更新为可变变量，就会出现一些编译器错误，这个修复需要一些更复杂的修改。为此，我们将在下一篇文章中继续，敬请期待！