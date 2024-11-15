
# 如何在大型Rust项目中设计错误处理

当Rust项目规模超过12个crate时，由于Rust的强类型特性，match会处理所有边缘情况，因此组织和处理错误会变得相当困难。

下面介绍一个用于在Rust项目中组织和处理错误的模式，到目前为止，它的可扩展性非常好！

为了说明这种模式，我们将以一个单一的模块化web应用程序为例，该应用程序使用不同的服务来处理业务领域的不同“有界上下文”。

![](../objInfo/assets/Pasted%20image%2020241115111843.png)

## 一个全局错误类型和多个本地错误类型

项目的核心错误类型是全局的，通常在项目根下的错误包中定义。

errs/error.rs
```rust
/// Error枚举表示面向用户的错误  
#[derive(Debug, Clone, thiserror::Error)]  
pub enum Error {  
    #[error("{0}")]  
    NotFound(String),  
    #[error("{0}")]  
    InvalidArgument(String),  
    #[error("Internal error.")]  
    Internal(String),  
    #[error("{0}")]  
    PermissionDenied(String),  
    #[error("{0}")]  
    AlreadyExists,  
    #[error("Authentication required")]  
    AuthenticationRequired,  
}
```

这种错误类型应该足够通用，可以处理应用程序可能遇到的所有类型的错误。它用于在有界上下文(服务)之间传播错误。

然后，每个服务定义自己的细粒度错误类型，并使用impl From< Error> for errs::Error定义特定于域的错误类型，以便本地错误可以使用"?"操作符。

services/users/error.rs
```rust
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("You must not be authenticated to perform this action")]
    MustNotBeAuthenticated,
    #[error("kernel: {0}")]
    Internal(String),
    #[error("Authentication required")]
    AuthenticationRequired,
    #[error("Password or email is not valid.")]
    PasswordOrEmailIsNotValid,
    #[error("User not found.")]
    UserNotFound,
    #[error("Session is not valid")]
    SessionIsNotValid,
    #[error("Permission denied")]
    PermissionDenied,

    // ...
}

impl From<Error> for errs::Error {
    fn from(value: Error) -> Self {
        match value {
            err @ Error::MustNotBeAuthenticated => errs::Error::PermissionDenied(err.to_string()),
            err @ Error::Internal(_) => errs::Error::Internal(err.to_string()),
            // ...
        }
    }
}
```

不同的服务不应该返回本地的错误类型。相反，它们必须返回全局errors::Error类型。本地错误不应该跨越上下文边界。

services/users/get_user.rs
```rust
impl UsersService {
    pub async fn get_user(
        &self,
        ctx: &RequestContext,
        input: GetUserInput,
    ) -> Result<User, errs::Error> {
        let current_user = ctx.user()?;

        // ...

        let user = self.repo.find_user(&self.db, input.id).await?;
        return Ok(user);
    }
}
```

这里，find_user返回类型为users:: Error的错误，如users:: error::Database或users:: error::UserNotFound，它不会在UsersService之外发生。而当使用“?”操作符时，转换为errors:: Error，如errors::Internal或errors::NotFound。

因此，当Sales服务调用get_user时，后者不必知道如何处理Users绑定上下文的所有特性，而是可以直接将错误传播到表示层。
```rust
impl SalesService {  
    pub async fn something(  
        &self,  
        ctx: &RequestContext,  
        input: SomethingInput,  
    ) -> Result<SomethingElse, errs::Error> {  
        // ...  
  
        // 这里没有错误处理，我们可以直接传播错误  
        let user = self.users.get_user(ctx, user_id).await?;  
  
        // ...  
    }  
}
```

## 集中错误处理

这种全局和局部错误模式在表示层进行集中错误处理。对于HTTP服务，它可以像下面这样简单：
```rust
#[derive(Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: String,
}

fn handle_error(err: errs::Error) -> HttpResponse {
    let (status, code) = match &err {
        Error::AuthenticationRequired => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"), // 401
        Error::PermissionDenied(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),          // 403
        Error::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),                  // 404
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),           // 500
    };

    let message = match &self {
        err @ Error::Internal(_) => {
            error!("{err}");
            err.to_string()
        }
        err @ _ => err.to_string(),
    };

    let res = ApiError {
        message,
        code,
    };
    return (status, Json(res)).into_response();
}

```
它可以减少错误处理逻辑的复杂性，并防止致命的错误，例如将内部错误泄漏给最终用户。

