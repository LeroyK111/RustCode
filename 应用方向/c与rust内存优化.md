# Rust和C语言中的神奇内存优化

编程语言中一些基本类型的内存大小(以Rust为例)：
贴心的附上: 8bits = 2nibble(半字节) = 1B(字节) = 1/1024 KB

- bool : 1B
- u8 : 1B
- u16 : 2B
- u32 : 4B
- u64 : 8B
- char : 4B

```rust
struct Foo {
    elem:  u32, 
    other: u16,
}
```

如果是6，就不会问你这个问题了。虽然6并不是完全不正确——因为结构体对于u32包含4个字节，对于u16包含2个字节——结构体的大小是通过对齐最大的字段并将总大小四舍五入到最接近的倍数来计算的。如果这不是很清楚，让我们看一个简单的表示。

再试一下:
```rust
struct Bar {
    num:     u16,
    bigger:  u32,
    another: u16,
}
```


## 对比c结构体

```rust
struct Bar {
    num:     u16,
    bigger:  u32,
    another: u16,
}
std::mem::size_of::<Bar>() // output: 8
```

```c
struct Bar {
    uint16_t num;
    uint32_t bigger;
    uint16_t another;
};

sizeof(struct Bar) // output: 12
```
似乎Rust编译器优化了结构体，而C编译器没有！
这是因为C程序员真的需要理解计算机是如何在底层(ABI)工作的.


但是重新排序后, c也获得相同的大小. 

### 这就是rust编译器为你做的效果, 结构体中字段的顺序可以显著影响数据布局和编译器优化；
```c
struct Bar {
    uint32_t bigger;
    uint16_t num;
    uint16_t another;
};

sizeof(struct Bar) // output: 8
```

### 将布尔值和1字节枚举存储在带外，以避免不必要的填充。

```rust
enum HtmlTag {
    H1,
    H2,
    UnorderedList,
    OrderedList,
    ...
}

std::mem::size_of::<HtmlTag>() // output: 1


struct HtmlToken {
    start_position: u32,
    token_tag:      HtmlTag, 
}
std::mem::size_of::<HtmlToken>() // output: 8
```

修改为: 是将枚举存储在带外。我们将使用由多个数组组成的数据结构。这意味着我们将使用两个数组并使用索引并行访问它们。在这种设计下，我们不会为每个新实例生成填充：
```rust
struct HtmlTokens {
    start_positions: [u32; 1],
    token_tags:      [HtmlTag; 1], 
}

std::mem::size_of::<HtmlTokens>() // output: 5
```
使用多数组技术，我们将每个HTML令牌的8字节实例转换为两个数组，每个数组只包含5字节的实例!以我们的HTML标记器为例，这种方法将内存中的标记列表大小减少了40%……对于这样简单的修改来说，这真是令人印象深刻。
