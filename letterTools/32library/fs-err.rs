// fs-errfs-err crate封装了std::fs中的错误信息，使其具有人类可读性。如果你使用过std::fs中的函数(如read_to_string或write)，你可能已经注意到，在出现错误的情况下，错误消息不是很有用：
let content = File::open("file-not-exist.txt")?;
let config = File::open("config-not-exist.txt")?;

// error message would be:
// The system cannot find the file specified. (os error 2)
// 使用fs-err crate，我们可以获得更详细的错误消息，例如哪个文件不存在：failed to open file `config-not-exist.txt`
    // caused by: The system cannot find the file specified. (os error 2)
