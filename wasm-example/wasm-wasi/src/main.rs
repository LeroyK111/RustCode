/*

# 添加跨平台插件
rustup target add x86_64-unknown-linux-gnu
cargo check --target x86_64-unknown-linux-gnu

# rust支持平台
rustc --print target-list

# 安装wasm服务插件,用以在浏览器之外运行wasm包
rustup target add wasm32-wasip1
cargo install wasmtime-cli

# 这将执行wasm文件
cargo run --target wasm32-wasip1
*/

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    println!("Hello, world!");
    #[cfg(target_arch = "wasm32")]
    println!("Hello, WebAssembly!");
}
