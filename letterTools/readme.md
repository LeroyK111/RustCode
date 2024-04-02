# 小工具合集

## 使用分位数的数值数据的无损压缩和解压缩工具。用法示例：
```rs

use pco::standalone::{auto_compress, auto_decompress};
use pco::DEFAULT_COMPRESSION_LEVEL;

fn main() {
  // your data
  let mut my_ints = Vec::new();
  for i in 0..100000 {
    my_ints.push(i as i64);
  }

  // Here we let the library choose a configuration with default compression
  // level. If you know about the data you're compressing, you can compress
  // faster by creating a `CompressorConfig`.
  let compressed: Vec<u8> = auto_compress(&my_ints, DEFAULT_COMPRESSION_LEVEL);
  println!("compressed down to {} bytes", compressed.len());

  // decompress
  let recovered = auto_decompress::<i64>(&compressed).expect("failed to decompress");
  println!("got back {} ints from {} to {}", recovered.len(), recovered[0], recovered.last().unwrap());
}
```

## 词性标注工具。使用示例：
```rs

use postagger::PerceptronTagger;

fn main() {
    let tagger = PerceptronTagger::new( "tagger/weights.json" , "tagger/classes.txt" , "tagger/tags.json" )  ; 
    let tags = tagger.tag( "the quick brown fox jumps over the lazy dog" ) ;
    for tag in &tags {
        println!( "{} {} {}" , tag.word , tag.tag , tag.conf ) ; 
    }
}
```



## ballast 一个简单的 API 负载测试工具，可用于比较 API 的性能。

```rs

```


## Quary

Quary是一个SQL引擎和CLI工具，可以让你轻松管理数据转换项目。Quary使团队能够设计、记录、测试和部署数据转换在你的数据存储上。

https://github.com/quarylabs/quary


## charybdis
用于ScyllaDB和Apache Cassandra的Rust ORM，Charybdis是scylla_rust_driver之上的ORM层，主要关注易用性和性能。
```sh
Charybdis的特性如下：

为整个模型的CRUD和复杂查询操作提供具有表现力的API



提供使用自动生成的partial_<model>!宏来处理模型字段子集的简单方法



通过使用自动生成的find_<model>!宏提供运行复杂查询的简便方法



自动迁移工具，分析src/model/*rs文件，并根据模型定义和数据库之间的差异运行迁移
```

## Creusot
Creusot是Rust代码的演绎验证器。它验证你的代码是否是安全的，不会出现恐慌、溢出和断言失败。通过添加注释，可以进一步验证代码是否正确。
Creusot通过将Rust代码翻译成WhyML来工作，WhyML是Why3的验证和规范语言。然后用户可以利用Why3的全功能(半)自动检验验证条件！

## pyo3

pyo3是Python的Rust绑定，包括用于创建本地Python扩展模块的工具。还支持从Rust二进制文件中运行Python代码并与之交互。

PyO3支持以下软件版本:

Python 3.7及以上版本(CPython和PyPy)



Rust 1.56及以上版本



你可以使用PyO3在Rust中编写本机Python模块，或者将Python嵌入到Rust二进制文件中。