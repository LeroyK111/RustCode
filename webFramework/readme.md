# web 常用框架系列

```
Actix、Axum、Rocket、Warp, Tide、Gotham、Nickel、Ntex, Tide, Poem

Pingora, Tower, tui-realm, governor, luminal, Tailcall, Salvo

Yew, Perseus, Sauron, Dioxus, Iced, Tauri, Loco
```




## 前景分析


### 前端

- WebAssembly (WASM)：WASM仍然是Rust前端功能的基石，可以使开发人员创建高性能和动态的用户界面。
    
- Yew：Yew是一个流行的框架，用于构建基于组件架构的前端应用程序。
    
- Seed：Seed是另一个强大的框架，专注于简单性和开发人员体验，使前端Rust入门变得容易。
    
- Percy：对于构建复杂的交互式数据可视化，Percy提供了一种声明性和高效的方法。


### 数据库和ORMs

- PostgreSQL和Diesel：这种强大的组合仍然是Rust中与关系数据库交互的流行选择。
    
- MongoDB和Monger：对于那些喜欢面向文档数据库的人来说，MongoDB和Monger库提供了一个健壮的解决方案。
    
- SQLx：这个多功能库提供编译时检查SQL查询，并支持各种数据库，包括PostgreSQL, MySQL和SQLite。

### 后端
- Rocket：Rocket仍然是Rust生态系统中领先的web框架，以其速度、安全性和易用性而闻名。
    
- Actix-web：Actix-web是另一个流行的选择，提供高性能和灵活的基于actor的架构。
    
- Warp：Warp以其闪电般的性能和有效处理大流量的能力而出名。
    
- Tide：Tide是一个较新的框架，它专注于为构建web服务提供最小且灵活的基础。
    
- Tower：Tower本身并不是一个框架，而是一个用于构建健壮且可组合的web服务的强大库

### 其他库和工具
- Serde：这个重要的库提供了强大而高效的序列化和反序列化功能，对于在web应用程序中处理数据至关重要。
    
- Reqwest：Reqwest是一个流行的HTTP客户端库，它可以很容易地与外部api和服务进行交互。
    
- Askama：Askama提供了一个用于生成动态HTML内容的模板引擎，简化了网页的创建。
    

有了如此多样化的可用工具，开发人员可以选择完美的组合来满足他们的特定需求和偏好。









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

