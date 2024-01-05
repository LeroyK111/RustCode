# web 常用框架系列

## Loco

```sh
cargo install loco-cli
```

创建新应用程序
```sh
loco new
```
这将询问项目名称(默认为myapp)，然后它将使用该名称创建一个文件夹。

它还会询问项目的类型，为你提供三个选择:

lightweight-service (最小版本，只有控制器和视图)


Rest API (包含数据库和用户认证)


Saas app (包含数据库和用户认证)


为简单起见，我们使用默认名称，并选择第一个不需要数据库的项目类型。

这样，就创建了一个名为myapp的文件夹，它是一个初始化的Rust Crate的git存储库，其中包含许多文件：

```tree
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── config
│   ├── development.yaml
│   ├── production.yaml
│   └── test.yaml
├── src
│   ├── app.rs
│   ├── bin
│   │   └── main.rs
│   ├── controllers
│   │   ├── home.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── views
│       ├── home.rs
│       └── mod.rs
└── tests
    ├── mod.rs
    └── requests
        ├── home.rs
        ├── mod.rs
        └── snapshots
            └── can_get_home@home_request.snap
```


启动开发服务器：
```sh
cargo loco start
```

我们也可以使用以下命令运行测试：
访问http://localhost:3000/api，网页显示“{"app_name":"loco"}”。
```sh
cargo test
```

下面，我们添加一个新的api。在src/views目录中，创建一个hello.rs文件，代码如下：
```rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloResponse {
    pub name: String,
}

impl HelloResponse {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn hello(&self) -> String {
        format!("Hello, {}", self.name)
    }
}
```


修改src/views/mod.rs文件：
```rs
pub mod hello;
pub mod home;
```

在src/controllers目录下，创建一个hello.rs文件，代码如下：
```rs
use loco_rs::prelude::*;

use crate::views::hello::HelloResponse;

async fn hello() -> Result<Json<String>> {
    format::json(HelloResponse::new("Loco").hello())
}

pub fn routes() -> Routes {
    Routes::new().add("/hello", get(hello))
}
```

最后，修改src/app.rs文件：
```rs
pub struct App;

#[async_trait]
impl Hooks for App {
    ......

    fn routes() -> AppRoutes {
        AppRoutes::empty()
            .prefix("/api")
            .add_route(controllers::home::routes())
            .add_route(controllers::hello::routes())
    }

    ......
}
```

启动服务器：
```sh
cargo loco start
```

访问http://localhost:3000/api/hello，网页显示"Hello, Loco"。