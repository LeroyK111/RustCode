// Bincode crate提供了字节数组结构的编码和解码。它使用一种紧凑的数据格式，适合存储在磁盘上，也适合在具有不同处理器架构的系统之间交换数据。
use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    number: i8,
    name: String,
}

fn check<T>(value: &T, expected_size: usize)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
    assert_eq!(encoded.len(), expected_size);

    let decoded: T = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(value, &decoded);
}

#[test]
fn test() -> Result<()> {
    let first_size = 9; // i8 + u64 for string length
    check(&Entity {number: 1, name: "".to_owned()}, first_size);
    let second_size = 15; // i8 + u64 for string length + 6 bytes of string
    check(&Entity {number: 2, name: "string".to_owned()}, second_size);
    Ok(())
}