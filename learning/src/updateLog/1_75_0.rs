/*
! Rust 编译器 1.75.0 版本
Rust编译器在两个不同的方面变得更快，使用了bolt a postlink优化器，性能提高了2%，并且还使用-Ccodegen-units=1来构建rust，这为LLVM中的优化提供了更多的机会。
*/





/*
?首先是Option::as_slice，在Rust 1.74中就有了Option转换为切片的功能，但还是不稳定的。而在Rust 1.75中工作得很好，所以你可以指定任何Option，然后使用as_slice将该Option转换为切片。
*/
fn sum_slice(arr: &[u32]) -> u32 {
    arr.iter().map(|value| value * 5).sum()
}

fn main() {
    let something = Some(3);
    // 切片
    let result = sum_slice(something.as_slice());
    println!("{result}");
}


/*
Trait中的方法返回impl Trait

?在Rust 1.75中，支持在Trait中使用async fn和-> impl Trait。我们先来看一个例子：
*/

trait Characters {
    fn characters(&self) -> impl Iterator<Item = char>;
}

struct ManyCharacters(String);

impl Characters for ManyCharacters {
    fn characters(&self) -> impl Iterator<Item = char> {
        self.0.chars()
    }
}

struct ManyNumbers(Vec<u8>);

impl Characters for ManyNumbers {
    fn characters(&self) -> impl Iterator<Item = char> {
        self.0.iter().map(|num| char::from(*num))
    }
}

fn demo() {
    let value = ManyCharacters("abcdefghijkl".to_string());
    for c in value.characters().enumerate() {
        println!("{:?}", c);
    }

    let value = ManyNumbers(vec![90, 83, 59, 200, 123, 100]);
    for d in value.characters().enumerate() {
        println!("{:?}", d);
    }
}

/*
?Trait中的异步函数
*/


trait JsonApi {
    type Error;
    async fn fetch(&self, url: &str) -> Result<serde_json::Value, Self::Error>;
}

struct MyApi {
    client: reqwest::Client,
}

impl JsonApi for MyApi {
    type Error = reqwest::Error;

    async fn fetch(&self, url: &str) -> Result<serde_json::Value, Self::Error> {
        let response = self.client.get(url).send().await?;
        Ok(serde_json::json!(response.text().await?))
    }
}


#[tokio::main]
async fn demo1() {
    let api = MyApi {
        client: reqwest::Client::new(),
    };

    let response = api.fetch("https://www.rust-lang.org/").await;

    println!("{}", response.unwrap());
}