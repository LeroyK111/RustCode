# IQ - 检索深层Rust结构 

IQ (Introspect Query) 让你通过简单的路径语法查询标准结构体、映射、枚举、数组、元组以及它们的嵌套组合，从而获取深层次的值。比如示例代码：

```rust
use iq::IQ;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize)]
struct Car {
    pub engine: String,
    pub passengers: Vec<Dog>,
    pub driver: Dog,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Dog {
    pub name: String,
    pub ears: u8,
}

...

// extract "primitive" values as strings with extract_primitive
assert_eq!(car.extract_primitive("driver.ears").unwrap(), "2");
assert_eq!(car.extract_primitive("driver.name").unwrap(), "Rex");
assert_eq!(car.extract_primitive("passengers.1.name").unwrap(), "Laïka");
assert_eq!(car.extract_primitive("passengers.1"), None); // it's not a primitive

// extract any value as Json with extract_json
assert_eq!(car.extract_json("wrong.path"), None);
assert_eq!(car.extract_json("driver.ears").unwrap(), "2");
assert_eq!(car.extract_json("driver.name").unwrap(), r#""Rex""#);
assert_eq!(
    car.extract_json("passengers.0").unwrap(),
    r#"{"name":"Roverandom","ears":1}"#
);
assert_eq!(car.extract_json("passengers.3"), None);

// extract any Deserializable value with extract_value
assert_eq!(
    car.extract_value("passengers.1").unwrap(),
    Some(Dog {
        name: "Laïka".to_string(),
        ears: 2
    }),
);

// You don't have to concat tokens if you build the path
assert_eq!(
    car.extract_primitive(vec!["passengers", "0", "ears"])
        .unwrap(),
    "1"
);

// Extract functions are available both as a trait and as standalone functions.
assert_eq!(iq::extract_primitive(&car, "driver.name").unwrap(), "Rex");
```