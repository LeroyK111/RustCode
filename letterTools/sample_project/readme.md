# 处理文件是软件工程中一个棘手但不可避免的部分，作为开发人员，经常需要从外部源加载信息以在项目中使用。

在这篇文章中，我们将学习如何用Rust编程语言读取JSON文件、YAML文件和TOML文件。

使用Serde框架解析JSON

Serde是一个框架，用于高效、通用地序列化和反序列化Rust数据结构。在本文的这一部分中，我们将使用serde crate来解析JSON文件。

Serde库的基本优点是，它允许你直接将数据解析为与源代码中类型匹配的Rust结构体。这样，你的项目在编译源代码时就知道传入数据的预期类型。


# 解析JSON文件

JSON格式是一种流行的用于存储复杂数据的文件格式，它是用于在web上交换数据的常用格式，并且在JavaScript项目中广泛使用。

我们可以通过静态类型方法和动态类型方法在Rust中解析JSON数据。

动态类型方法最适合于不确定JSON数据格式是否符合源代码中预定义的数据结构体的情况，而静态类型方法则适用于确定JSON数据格式的情况。

要开始使用，必须安装所有必需的依赖项。首先，我们在Cargo.toml文件中添加serde和serde_json crate作为依赖项。除此之外，确保启用了可选的派生功能，这将帮助我们生成(反)序列化的代码：

## 动态解析JSON

首先，我们编写一个use声明来导入serde_json crate。Value enum是serde_json crate的一部分，它代表任何有效的JSON值——可以是字符串、空值、布尔值、数组等。

在根目录中，我们将创建一个.json文件来存储任意JSON数据，我们将读取数据并将其解析为源代码中定义的结构体。创建一个data文件夹，然后创建一个sales.json文件并使用此json数据更新它：

```json
{
  "products": [
    {
      "id": 591,
      "category": "fruit",
      "name": "orange"
    },
    {
      "id": 190,
      "category": "furniture",
      "name": "chair"
    }
  ],
  "sales": [
    {
      "id": "2020-7110",
      "product_id": 190,
      "date": 1234527890,
      "quantity": 2.0,
      "unit": "u."
    },
    {
      "id": "2020-2871",
      "product_id": 591,
      "date": 1234567590,
      "quantity": 2.14,
      "unit": "Kg"
    },
    {
      "id": "2020-2583",
      "product_id": 190,
      "date": 1234563890,
      "quantity": 4.0,
      "unit": "u."
    }
  ]
}
```

现在我们有了数据，在main.rs文件中写入以下代码：
```rust
use serde_json::Value;
use std::fs;

fn main() {
    let sales_and_products = {
        let file_content =
            fs::read_to_string("./data/sales.json").expect("LogRocket: error reading file");
        serde_json::from_str::<Value>(&file_content).expect("LogRocket: error serializing to JSON")
    };
    println!(
        "{:?}",
        serde_json::to_string_pretty(&sales_and_products)
            .expect("LogRocket: error parsing to JSON")
    );
}
```

from_str将一个连续的字节片段作为参数，并根据JSON格式的规则，从中反序列化一个Value类型的实例。

在实际项目中，除了显示输出外，我们还希望访问JSON数据中的不同字段，操作数据，甚至尝试将更新的数据存储在相同或不同的文件中。

考虑到这一点，让我们尝试访问sales_and_products变量上的字段，更新其数据，并可能将其存储在另一个文件中：
```rs
use serde_json::{Number, Value};
use std::fs;

fn main() {
    let mut sales_and_products = {
        let file_content =
            fs::read_to_string("./data/sales.json").expect("LogRocket: error reading file");
        serde_json::from_str::<Value>(&file_content).expect("LogRocket: error serializing to JSON")
    };

    if let Value::Number(quantity) = &sales_and_products["sales"][1]["quantity"] {
        sales_and_products["sales"][1]["quantity"] =
            Value::Number(Number::from_f64(quantity.as_f64().unwrap() + 3.5).unwrap());
    }
    fs::write(
        "./data/sales.json",
        serde_json::to_string_pretty(&sales_and_products)
            .expect("LogRocket: error parsing to JSON"),
    )
    .expect("LogRocket: error writing to file");
}
```

在上面的代码片段中，我们利用Value::Number变量对sales_and_products进行模式匹配["sales"][1]["quantity"]，我们期望它是一个数值。

使用Number结构体上的from_f64函数，将操作quantity.as_f64().unwrap() + 3.5返回的f64值转换回Number类型，然后将其存储回sales_and_products中["sales"][1]["quantity"]并更新了它的值(注意：确保sales_and_products是一个可变变量)。

然后，使用write函数和文件路径作为参数，使用serde_json::to_string_pretty函数的结果值创建并更新一个文件。这个结果值将与我们之前在终端上输出的相同，但格式良好。


## 静态解析JSON

另一方面，如果我们确定JSON文件的结构，我们可以利用不同的方法，包括在项目中使用预定义的数据。

静态版本的源代码在开头声明了三个结构体：
```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>
}
#[derive(Deserialize, Serialize, Debug)]
struct Product {
    id: u32,
    category: String,
    name: String
}
#[derive(Deserialize, Serialize, Debug)]
struct Sale {
    id: String,
    product_id: u32,
    date: u64,
    quantity: f32,
    unit: String
}
```
main.rs的代码如下：
```rust
use std::{fs, io};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), io::Error> {
    let mut sales_and_products: SalesAndProducts = {
        let data = fs::read_to_string("./data/sales.json").expect("LogRocket: error reading file");
        serde_json::from_str(&data).unwrap()
    };
    sales_and_products.sales[1].quantity += 1.5;
    fs::write(
        "./data/sales.json",
        serde_json::to_string_pretty(&sales_and_products).unwrap(),
    )?;

    Ok(())
}
```
与我们的动态方法相比，源文件的其余部分保持不变。


## 静态解析TOML

我们将静态地读取和解析这个TOML文件，这意味着我们知道TOML文件的结构，并且我们将在本节中使用预定义的数据。

首先，在Cargo.toml文件中加入toml依赖项：
```toml
[dependencies]
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0"
toml = "0.8.12"
```

```rust
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug, Serialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct Redis {
    host: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct Sqlite {
    db_file: String
}

#[derive(Deserialize, Debug, Serialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String
}

#[derive(Deserialize, Debug, Serialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql
}
```

我们定义了每个结构体来映射到TOML文件中的内容，结构体中的每个字段映射到表/头下的键/值对。

接下来，我们在main函数体中使用serde, serde_json和toml crate来读取和解析config.toml文件：

```rust
fn main() {
    let config: Config = {
        let config_text =
            fs::read_to_string("./data/config.toml").expect("LogRocket: error reading file");
        toml::from_str(&config_text).expect("LogRocket: error reading stream")
    };
    println!("[postgresql].database: {}", config.postgresql.database);
}
```

## 静态解析YAML

项目中使用的另一个流行的配置文件格式是YAML文件。在本节中，我们将静态地读取和解析Rust项目中的YAML文件。

创建一个data/configuration.yaml文件，内容如下：

我们将使用config crate来解析YAML文件，我们需要定义必要的结构体来充分解析YAML文件的内容：

```rust
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
```

接下来，我们在main函数中读取和解析YAML文件：
```rust
fn main() -> Result<(), config::ConfigError> {
    let mut settings = config::Config::default();
    let Settings {
        database,
        application_port,
    }: Settings = {
        settings.merge(config::File::with_name("data/configuration"))?;
        settings.try_deserialize()?
    };

    println!("{}", database.connection_string());
    println!("{}", application_port);
    Ok(())
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
```
让我们来分解一下上面的代码：

我们使用字段类型的默认值初始化Config结构体。


使用config::File::with_name函数，我们搜索并定位具有该名称和配置的YAML文件。根据文档的定义，我们使用Config结构体上的merge函数合并配置属性源。


将YAML文件内容解析为我们定义的Settings结构体


格式化并返回Postgres连接字符串