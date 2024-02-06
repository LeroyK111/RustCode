# 读写文件的五种方法

## 将整个文件读入到字符串

这种方法除了处理文件和处理其内容之外，不需要担心任何事情。将整个文件读入String的优点：
可以处理包含字符串内容的文件

可以一次整体处理

另一方面，这种方法也有它的缺点：
过大的文件可能会对性能产生严重影响

文件越大，程序的内存消耗就越大

包含二进制内容的文件不能以这种方式处理

下面的例子展示了如何将整个文件读入String：
```rust
use std::fs;  
  
fn read_file_content_as_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {  
    let string_content = fs::read_to_string(path)?;  
    Ok(string_content)  
}
```

## 将整个文件读入到字节向量
如果不处理String内容，但需要处理某种形式的二进制格式，则可以将整个文件读入字节向量。不过，这个方法仍然适用于字符串内容。你必须自己实例化它，而不是直接从方法调用中接收String。如果你不处理字符串内容，则不需要这样做。

这个方法的优点是：
可以处理包含任何形式内容的文件

可以一次处理整个文件
缺点是：
文件太大可能会对性能产生严重影响

文件越大，程序的内存消耗就越大
下面的例子演示了如何将整个文件读入字节向量：
```rust
use std::fs;  
  
fn read_file_as_bytes(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {  
    let byte_content = fs::read(path)?;  
    Ok(byte_content)  
}


```
如果将字节向量转换为String，可以这样做：
```rust
use std::fs;  
use std::str;  
  
fn read_file_as_bytes(path: &str) -> Result<String, Box<dyn std::error::Error>> {  
    let byte_content = fs::read(path)?;  
    let string_content = str::from_utf8(&byte_content)?;  
  
    Ok(string_content.to_string())  
}
```


## 逐行读取文件

如上所述，如果处理大文件，一次读取整个文件可能会导致问题。在这种情况下，最好使用逐行方法处理这些文件。当然，这主要适用于具有String内容的文件。

Rust在其标准库中有一个方便的结构体，它去掉了一些较低级别的细节，称为BufReader。这种方法可以处理以下特点的文件：
包含字符串内容的文件

不能一次处理太大的文件


然而，这种方法也有一些缺点：
它只适用于字符串内容的文件

实现可能很快变得更加复杂

根据文件的格式，如果不是要处理的所有内容都放在同一行，则可能需要自己缓冲行

下面的示例展示了如何逐行读取文件：
```rust
use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_file_line_by_line(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            // line是字符串
            Ok(line) => process_line(line),
            Err(err) => handle_error(err),
        }    
    }

    Ok(())
}
```

## 以单个字节逐步读取文件

前一种方法是逐行读取文件，而将要介绍的这种方法允许你从BufReader处理的文件中读取单个字节。

使用这种方法你需要：
需要完全控制文件内容的处理

自己实现大量的内容处理

自己处理缓冲，如果一次读取所有文件内容会使内存消耗爆炸


它的缺点包括：
你必须处理原始数据。在这种情况下，它甚至是单个原始字节

你可能仍然需要一个缓冲区来临时保存单个字节，直到可以将多个字节合并为更有意义的内容


下面的例子演示了如何以单个字节逐步读取文件：
```rust
use std::fs::File;
use std::io::{BufReader, Read};

fn read_file_as_single_bytes(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for byte in reader.bytes() {
        match byte {
            // byte正好是一个字节
            Ok(byte) => process_byte(byte),
            Err(err) => handle_error(err),
        }
    }

    Ok(())
}
```

## 以字节块读取文件

如果需要更大的灵活性，可以使用BufReader从文件中读取块。说实话，BufReader也在底层进行了优化，当使用它的.bytes()方法时，它不会单独读取每个字节。它以块的形式读取它们，然后从Iterator返回单个字节。

但是，当你想要自己处理块时，这并没有多大帮助。当然，也可以在使用bytes()时手动缓冲字节。

像其他方法一样，以字节块的形式读取文件内容既有优点也有缺点。它的优点是：
可以完全控制如何处理文件的内容

提供了最大的灵活性，因为可以动态调整块大小并对特定情况做出反应

如果必须处理大文件，读取所有文件内容将使内存消耗爆炸，则可以使用这种方法。


当然，这种方法也存在一些已知的缺陷：
必须处理原始数据，所有的解码和处理都由你来决定

针对特定场景，可能需要进行几次尝试来优化缓冲区大小

如果块太小，实际上可能会损害程序的整体性能(太多的系统调用)。


下面的例子展示了如何以字节块的形式读取文件：
```rust
use std::fs::File;
use std::io::{BufReader, BufRead}

const BUFFER_SIZE: usize = 512;

fn read_file_in_byte_chunks(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);

    loop {
        let buffer = reader.fill_buf()?;

        let buffer_length = buffer.len();

        if buffer_length == 0 {
            break;
        }

        do_something_with(buffer);

        // 冲缓冲区中消耗所有字节
        reader.consume(buffer_length);
    }

    Ok(())
}
```
读取文件是开发软件时常见的操作，本文介绍了在Rust中读取文件(包括字符串和原始二进制格式)的五种常用方法。所有方法都有优点和缺点，需要选择适合你的特定情况和用例的方法。

如果是小文件并处理String内容，将整个文件读入String是一个很好的选择。另一方面，如果文件变大或者根本不处理String内容，则该方法不是最好的。

如果文件很小，并且要处理任意的原始内容，那么将整个文件读入字节向量是一个不错的选择。但是，如果文件变大并且有内存限制，则不能使用此功能。

如果处理String内容并且不希望内存增长太多，那么逐行读取文件是一个很好的选择。如果不处理String内容，并且文件将想要的内容分散到多行，那么该方法就不够用了，这需要你自己缓冲行。

以单个字节逐步读取文件是最基本的方法之一。如果你想要灵活性和大量的控制，这是一个很好的选择。另一方面，如果需要将多个字节合并为更有意义的内容，可能还要自己进行数据缓冲。

最后，以字节块读取文件比单独读取每个字节要灵活一些。它提供了对数据处理的完全控制，也可以动态调整。但同样，需要处理原始数据，并且可能需要一些时间来微调分块。