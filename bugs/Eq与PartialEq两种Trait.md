最初，我很难理解为什么rust用Eq与PartialEq这两个Trait来重载 == 和 != 操作符，它们有什么不同的用途？

首先，如果满足以下属性，任何类型T都可以实现PartialEq：

对称性：a == b 意味着 b == a


传递性：a == b和b == c 意味着 a == c


而类型T实现Eq，除了满足上面的属性，还需要满足下面的属性：

自反性：a == a


因此，Eq是PartialEq的子trait，这意味着类型T必须在实现Eq之前实现PartialEq。

但是，为什么Eq trait没有提供任何方法，或者说它只是一种标记，Eq trait起到了什么作用？

假设我们想要表示一个学生。我们可以创建一个简单的结构体：

```rust
struct Student {
  id: u64,
  address: String,
}
```

现在，我们想比较两个学生实例，如果他们有匹配的id，即使他们有不同的地址(多个学生可以有相同的地址)，我们也假设两个学生实例相等。因此，我们可以为Student类型实现PartialEq trait。

根据目前的要求，Student类型满足上面提到的所有三个属性，因此也可以为Student类型实现Eq trait：

```rust
impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Student { }
```

但问题是，即使没有按照要求生成唯一id，学生实例也可以存在。此外，我们还想，没有id的学生，都可以用id等于0来表示。

但这会导致另一个问题，如果两个学生的id为0，这意味着这两个学生相同，但是他们实际上不相等。

我们现在需要修改我们的PartialEq impl：

```rust
impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        if self.id == 0 || other.id == 0 {
            false
        } else {
            self.id == other.id
        }
    }
}
```
现在，如果一个或两个student id都是0，它们不可能相等。但是，我们现在不满足第三个条件，即自反性：a == a。因此，Student类型不应该实现Eq trait。

在代码层面，没有什么可以阻止我们为我们的类型实现Eq trait，因为它只是一个标记trait，编译器不能确认student是否包含所有三个属性。

但是，在语义层面，我们不应该为Student实现Eq trait。

当我们将Student与仅限制必须实现Eq的类型一起使用时，编译器就会提示。HashMap键需要实现Eq trait，因为HashMap的键必须满足自反关系。在我们的例子中，多个学生实例的id都可以是0，因此Student类型不能作为键。

其中一个流行的例子是rust中的浮点数。f64不能用作HashMap的键，你可以自己验证。

现在我们明白Eq即使是一个没有任何方法的标记Trait，它也有非常重要的使用场景。