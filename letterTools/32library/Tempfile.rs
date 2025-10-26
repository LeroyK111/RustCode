// Tempfile crate提供了一个用于创建临时文件和目录的API。// Write
let mut tmpfile: File = tempfile::tempfile().unwrap();
write!(tmpfile, "Hello World!").unwrap();

// Seek to start
tmpfile.seek(SeekFrom::Start(0)).unwrap();

// Read
let mut buf = String::new();
tmpfile.read_to_string(&mut buf).unwrap();
assert_eq!("Hello World!", buf);