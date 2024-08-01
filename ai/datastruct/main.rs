// 二进制
use datastruct::binary_util::Binary;
use std::path::PathBuf;

// Create a new Binary instance from a vector of bytes
let binary = Binary::new(vec![72, 101, 108, 108, 111]);

// Read binary data from a file
let path = PathBuf::from("path/to/file");
let binary_from_file = Binary::from_file(path)?;

// Decode base64-encoded string to binary
let base64_string = "SGVsbG8gd29ybGQ=";
let binary_from_b64 = Binary::from_b64(base64_string.to_string())?;

// DValue Enum
use datastruct::DValue;
use std::collections::HashMap;

// Create different DValue instances
let string_value = DValue::String("Hello World".to_string());
let number_value = DValue::Number(42.0);
let boolean_value = DValue::Boolean(true);
let list_value = DValue::List(vec![DValue::Number(1.0), DValue::Number(2.0)]);
let dict_value = DValue::Dict(HashMap::new());

// Convert a DValue instance to JSON
let json_string = string_value.to_json();

// Parse a string to a DValue instance
let parsed_value = DValue::from("b:SGVsbG8gV29ybGQ=:");
let size = string_value.size();