/*
文件处理
*/

// 使用Tokio处理文件
// 向文件写入数据
// 让我们从一个简单但重要的任务开始：将数据异步写入文件。save_bytes_to_file函数演示了如何使用Tokio完成此操作。

use std::io;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, AsyncReadExt, AsyncSeekExt};
use tokio::fs;


/*
这里，我们创建一个由input_path指定的文件，并将提供的数据异步写入该文件。Tokio的AsyncWriteExt trait提供了write_all方法，简化了异步写操作。
*/
pub async fn save_bytes_to_file(data: &[u8], input_path: &str) -> io::Result<()> {
    let mut file = File::create(input_path).await?;
    file.write_all(data).await?;
    Ok(())
}

/*
从文件中异步读取数据遵循类似的模式，load_bytes_from_file函数演示了如何实现这一点：
在这个函数中，打开input_path指定的文件，使用read_to_end异步读取其内容，并将读取的数据作为字节向量返回。
*/
pub async fn load_bytes_from_file(input_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(input_path).await?;
    let mut contents = vec![];

    file.read_to_end(&mut contents).await?;
    Ok(contents)
}



/*
异步文件查找和读取

Tokio还支持异步文件查找和读取操作。使用read_portion_of_file函数，它异步读取文件的一部分：
*/
pub async fn read_portion_of_file(file_path: &str, start: u64, end: u64) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path).await?;
    let mut buffer = vec![0; (end - start) as usize];

    file.seek(io::SeekFrom::Start(start)).await?;
    file.read_exact(&mut buffer).await?;

    Ok(buffer)
}



/*
处理文件块

在某些情况下，可能需要以固定大小的块从文件中读取数据。read_chunks_sizes_of_file函数演示了如何实现这一点：
这个函数在一个循环中从文件读取数据块，异步处理每个数据块。
*/

pub async fn read_chunks_sizes_of_file(file_path: &str) -> io::Result<Vec<u32>> {
    let mut sizes: Vec<u32> = Vec::new();
    let mut file = File::open(file_path).await?;
    let mut buffer = [0u8; 4];

    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        let converted_u32_from_bytes = u32::from_ne_bytes(buffer);
        sizes.push(converted_u32_from_bytes);
        file.seek(io::SeekFrom::Current(converted_u32_from_bytes as i64)).await?;
    }

    Ok(sizes)
}


/*
向文件追加数据

在Tokio中异步地向文件追加数据是很简单的，append_to_file函数说明了这一点：
在这个函数中，我们以追加模式打开文件，并在文件末尾异步写入所提供的数据。
*/

pub async fn append_to_file(file_path: &str, data: &[u8], create_file: bool, add_bytes_size: bool) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(create_file)
        .open(file_path)
        .await?;

    if add_bytes_size {
        let data_length = data.len() as u32;
        let mut tmp_buffer = [0u8; 4];
        tmp_buffer.copy_from_slice(&data_length.to_le_bytes());
        file.write_all(&tmp_buffer).await?;
    }

    file.write_all(data).await?;
    Ok(())
}

/*
文件是否存在和文件大小

最后，Tokio简化了检查文件存在和异步获取文件大小的过程。函数file_exists和get_file_size演示了这个例子：
*/


pub async fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).await.is_ok()
}

pub async fn get_file_size(file_path: &str) -> u64 {
    if let Ok(metadata) = fs::metadata(file_path).await {
        metadata.len()
    } else {
        0
    }
}

/*
Tokio在文件读取中的局限性

Tokio在读取大量文件方面可能没有提供显著优势的一个关键原因是操作系统的本机接口中缺少异步文件api。虽然Tokio擅长管理异步任务和I/O操作，但由于在操作系统级别缺乏对异步文件访问的支持，它在处理文件操作时的有效性受到限制。

线程池效率

在以读取大量文件为主要任务的场景中，利用普通线程池通常可以产生与使用Tokio相当的性能。线程池有效地跨多个线程分发任务，支持并发文件读取，而无需依赖本地异步文件api。这种方法可以提供类似级别的并行性和效率，而不会增加集成Tokio异步运行时的复杂性。

复杂度开销

将Tokio集成到代码库中会引入额外的复杂性，特别是当主要关注文件操作时。对于主要涉及同步或批处理文件读取而没有广泛异步协调的任务，采用Tokio可能会增加不必要的复杂性，而不会带来相应的性能提升。在这种情况下，选择更简单的并发模型(例如普通线程池)可能更合适，也更易于管理。

资源利用率

Tokio的异步运行时旨在有效地管理线程和I/O操作等资源。然而，在文件读取构成大部分工作负载且异步协调最小的场景中，Tokio运行时管理的开销可能会超过它的好处。这可能导致资源利用率低于最佳，并可能影响性能，特别是与普通线程池等更直接的并发模型相比。


总结

虽然Tokio仍然是异步编程和处理I/O任务的强大工具，但在同步读取大量文件时，它的优势可能无法完全实现。在异步文件api不可用且主要任务围绕同步文件I/O的情况下，利用普通线程池或其他并发模型可以在复杂性较低的情况下提供相当的性能。仔细评估特定的需求和所涉及的权衡，以确定有效处理文件的最合适解决方案，这一点至关重要。
*/