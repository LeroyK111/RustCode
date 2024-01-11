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