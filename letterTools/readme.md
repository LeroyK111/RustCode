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