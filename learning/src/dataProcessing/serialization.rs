/*
! 提高Rust序列化性能
*/

struct Name {
    first_name: String,
    last_name: String,
}

use std::fmt::{self, Display};

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let name = Name {
        first_name: "Max".to_string(),
        last_name: "Mustermann".to_string(),
    };

    println!("Hello {name}");
    // Output: Hello Max Mustermann
}

// 简单的方法是将Name转换为全名字符串，然后对其进行序列化：
fn naive(names: &[Name]) -> serde_json::Result<String> {
    /*
        我们迭代Name切片，并使用to_string()方法将Name转换成Display字符串表示形式。然后，然后使用.collect::<Vec<_>>()方法转换成向量，最后序列化它。

    这里的问题是我们为每个Name执行to_string()方法时，必须在堆上分配的String，堆分配是昂贵的！

    其实，不必在序列化之前创建字符串。我们可以在序列化过程中创建它们，并直接将它们附加到序列化器的缓冲区中，而不是先分配我们自己的缓冲区。

        */
    let full_names = names
        .iter()
        .map(|name| name.to_string())
        .collect::<Vec<_>>();

    serde_json::to_string(&full_names)
}

/*
实现更好的序列化
*/

use serde::{Serialize, Serializer};

#[derive(Serialize)]
struct Name {
    first_name: String,
    last_name: String,
}


/*
我们告诉序列化器从实例中“收集”一个字符串。collect_str接受一个参数&T，其中T实现Display并将该Display表示的字符串追加到序列化器。

我们在Display和Serialize trait之间建立了一个桥梁，现在Name类型实现了Serialize，我们可以直接将它的切片传递给Json序列化器：
*/
impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self)
    }
}


fn manual_serialize(names: &[Name]) -> serde_json::Result<String> {
    serde_json::to_string(names)
}

// 通过在类型Name上直接手动实现Serialize trait， 提高了序列化的性能。但是我们失去了拥有默认派生的能力，即不能使用#[derive(Serialize)]。


/*
! 序列化性能2
*/



// 此泛型格式器封装了实现Display的类型T，并使用该表示对T进行序列化。你不仅可以在Name类型上使用它，还可以在任何实现Display的类型上使用它。
struct DisplayFormatter<T: Display>(T);

impl<T: Display> Serialize for DisplayFormatter<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0)
    }
}
// 但是，有时你的类型并不一定要实现Display，或者你想要不同的Display和Serialize实现。在这种情况下，可以使用一个封装具体类型的格式化器：
struct FullNameFormatter<'a>(&'a Name);

impl<'a> Serialize for FullNameFormatter<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format_args!("{} {}", self.0.first_name, self.0.last_name))
    }
}
/*
这里，我们直接使用format_args!宏，像println!宏和format!宏在底层使用的就是format_args!宏。它返回Arguments，重要的是Arguments实现了collect_str()方法所需的Display。

现在，当我们想要序列化Name时，我们可以使用该格式化器：
*/

fn formatter(names: &[Name]) -> serde_json::Result<String> {
    let full_names = names.iter().map(FullNameFormatter).collect::<Vec<_>>();

    serde_json::to_string(&full_names)
}

// 查看基准测试结果，可以看到该方法与之前的方法几乎具有相同的性能：
/*
serialization            fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ ser_formatter                       │               │               │               │         │
│  ├─ 0                  282.2 ms      │ 502 ms        │ 294 ms        │ 312.4 ms      │ 100     │ 100
..........    
│  ╰─ 20                 99.15 ms      │ 222.2 ms      │ 110.4 ms      │ 116.2 ms      │ 100     │ 100
╰─ ser_manual_serialize                │               │               │               │         │
   ├─ 0                  172 ms        │ 299.8 ms      │ 175.6 ms      │ 184 ms        │ 100     │ 100
..........
   ╰─ 20                 95.45 ms      │ 127.9 ms      │ 99.34 ms      │ 100.7 ms 
*/

/*
几乎没性能差异，因为封装器类型在Rust中是零成本抽象。

但实际上，应该有一个与封装器类型本身无关的额外成本。在上面的测试结果中应该会看到，对于低N值，此方法的性能略低于manual_serialize

这是因为有一次收集map数据并进行Vec的分配！这种分配几乎没有出现在基准测试结果中，特别是对于较高的N值。这是因为它只执行一次，与序列化本身相比，它的开销可以忽略不计。

接下来，我们将取消Vec分配，虽然这似乎是一个不必要的优化，但至少可以看到另一个格式化程序示例。
*/

/*
序列格式化器

我们不能跳过收集map数据并将Iterator传递给serde_json序列化器的过程，因为Serde库不直接支持Iterator的序列化。Serialize trait没有被Iterator实现，但是Serde的Serializer提供了collect_seq方法来收集Iterator。

我们需要一个封装器类型，它接受slice并通过将map后的数据传递给collect_seq来进行序列化：
*/
struct FullNameSequenceFormatter<'a>(&'a [Name]);

impl<'a> Serialize for FullNameSequenceFormatter<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.0.iter().map(FullNameFormatter))
    }
}

// 现在我们可以将序列格式化器传递给serde_json：
fn sequence_formatter(names: &[Name]) -> serde_json::Result<String> {
    serde_json::to_string(&FullNameSequenceFormatter(names))
}
/*
serialization              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ ser_manual_serialize                  │               │               │               │         │
│  ├─ 0                    170.6 ms      │ 358.9 ms      │ 185.8 ms      │ 193.5 ms      │ 100     │ 100
............
│  ╰─ 20                   97.34 ms      │ 143.5 ms      │ 101.2 ms      │ 106.2 ms      │ 100     │ 100
╰─ ser_sequence_formatter                │               │               │               │         │
   ├─ 0                    172.8 ms      │ 225.5 ms      │ 183.3 ms      │ 186.7 ms      │ 100     │ 100
............
   ╰─ 20                   97.89 ms      │ 152.6 ms      │ 99.4 ms       │ 102 ms        │ 100     │ 100
*/
// 这才是真正的没有性能差异！

/*
“避免分配”是这两篇文章的真正结论，更具体地说，应该是避免在序列化之前分配数据的中间状态。

我们已经看到，Serialize Trait的简单手工实现可以带来很大的性能改进。但这并不意味着应该总是手动实现Serialize Trait，只有需要自定义序列化时才应该手动实现。否则，只需在类型上使用#[derive(Serialize)]即可。
*/