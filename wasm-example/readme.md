# 在Rust中编写WASM的三种方式

在这篇文章中，我们将讨论如何在Rust中编写WebAssembly模块。WebAssembly是可移植编译目标，能够方便地在web上与JavaScript互操作。Rust能够利用这一点，使得它在许多用例中非常有用。例如：

- CPU密集型工作(加密)
    
- GPU密集型工作(图像/视频处理、图像识别)
    
      
    

本文将重点介绍使用Rust编写图像处理的WASM模块，并探讨部署WASM的常用方法。

我们将重点尝试用三种不同的方式编写WASM模块：

- 使用wasm-bindgen-cli
    
- 使用wasm-pack
    
- 使用napi-rs
    

  

我们将首先使用wasm-bindgen-cli来创建我们的应用程序，然后使用wasm-pack。本文的重点是创建一个简单的图像处理模块，字节数组操作和数据处理是Rust可以显著提高应用程序速度的领域。

在开始之前，请确保安装了wasm32-unknown-unknown。如果没有，使用以下命令安装：

```
rustup target add wasm32-unknown-unknown
```

请注意，为了测试我们的模块，还需要额外安装npm(或任何替代方案)



## 构建项目

首先使用如下命令创建一个名为wasm-example的新项目：

```
cargo init --lib wasm-example
```

然后，执行以下命令来安装依赖项：

```
cargo add wasm-bindgen@0.2.91cargo add js-sys@0.3.68cargo add image@0.25.0
```

我们还需要将动态库标志添加到Cargo.toml文件中。通常，它让Cargo知道我们想要创建一个动态系统库——但是当它与WebAssembly目标一起使用时，它的意思是“创建一个没有启动函数的*.wasm文件”：

```
[lib]crate-type = ["cdylib"]
```

为了能够在Rust中使用JavaScript类型，除了使用wasm-bindgen宏之外，我们还需要使用extern C。这允许我们直接从JavaScript中导入函数到Rust中！

WASM中的Hello World例子是这样的：

```
use wasm_bindgen::prelude::*;#[wasm_bindgen]extern "C" {    fn alert(s: &str);}#[wasm_bindgen]pub fn greet(name: &str) {    alert(&format!("Hello, {}!", name));}
```

注意，外部C代码中的alert函数直接取自JavaScript，因此我们可以在Rust函数中调用它。如果我们要编译它并在JavaScript文件中执行，它将与从常规JavaScript文件中调用alert()相同。

我们可以应用相同的逻辑来处理其他类型和函数——缓冲区。JavaScript中的Vec可以用以下两种方式表示：

- Uint8Array类型(直接相当于JavaScript中的Vec)
    
- Buffer类型
    

Buffer是Uint8Array的子类。这是因为当Node.js第一次发布时，没有Uint8Array类型-这就是导致Buffer类型创建的原因。后来，当Uint8Arrays被引入ES6时，两者最终被合并。许多JavaScript库仍然使用Buffer，通过使用js-sys，我们可以在JavaScript和Rust之间实现互操作性。

让我们在src/lib.rs文件中一个定义Buffer类型和提供一个方法buffer()方法：

```rust
use js_sys::{wasm_bindgen, ArrayBuffer};  
  
// 这定义了Node.js的Buffer类型  
#[wasm_bindgen]  
extern "C" {  
    pub type Buffer;  
  
    #[wasm_bindgen(method, getter)]  
    fn buffer(this: &Buffer) -> ArrayBuffer;  
  
    #[wasm_bindgen(method, getter, js_name = byteOffset)]  
    fn byte_offset(this: &Buffer) -> u32;  
  
    #[wasm_bindgen(method, getter)]  
    fn length(this: &Buffer) -> u32;  
}
```

现在，当我们编写WASM函数时，我们可以直接引用Buffer类型！

让我们编写用于转换图像文件格式的Rust函数。这个函数需要上面定义的缓冲区，它返回Vec<>。当我们通过wasm-pack或其他编译器编译它时，它将自动转换为Uint8Array。

```rust
use image::io::Reader;  
use image::ImageFormat;  
use js_sys::{wasm_bindgen, ArrayBuffer, Uint8Array};  
use std::io::Cursor;  
use wasm_bindgen::prelude::wasm_bindgen;  
  
// .. extern C stuff goes here  
  
#[wasm_bindgen]  
pub fn convert_image(buffer: &Buffer) -> Vec<u8> {  
    // 这将从Node.js缓冲区转换为Vec<u8>  
    let bytes: Vec<u8> = Uint8Array::new_with_byte_offset_and_length(  
        &buffer.buffer(),  
        buffer.byte_offset(),  
        buffer.length(),  
    )  
    .to_vec();  
  
    let img2 = Reader::new(Cursor::new(bytes))  
        .with_guessed_format()  
        .unwrap()  
        .decode()  
        .unwrap();  
  
    let mut new_vec: Vec<u8> = Vec::new();  
    img2.write_to(&mut Cursor::new(&mut new_vec), ImageFormat::Jpeg)  
        .unwrap();  
  
    new_vec  
}
```

## 通过wasm-bindgen-cli构建

首先，执行以下命令安装wasm-bindgen-cli：

```
cargo install wasm-bindgen-cli
```

然后，我们需要构建wasm32-unknown-unknown target，将Rust代码编译到WASM，我们可以这样做：

```
cargo build --target=wasm32-unknown-unknown
```

接下来，需要使用wasm-bindgen命令来生成JS粘合代码，以使其正常工作。我们使用nodejs target，它将生成一个CommonJS模块，并将其放在./pkg文件夹中，然后就可以将其植入到任何我们想要的地方。

```
wasm-bindgen --target nodejs --out-dir ./pkg ./target/wasm32-unknown-unknown/debug/wasm_example.wasm
```

现在我们可以将WASM代码作为包发布。

如果你不想使用CommonJS，比如你正在使用ESM (EcmaScript模块，或ES6模块)，CLI目前支持生成以下几种target：

- bundler：生成用于Webpack等捆绑器的代码
    
- Web：可直接在Web浏览器中加载
    
- nodejs：可通过require作为CommonJS的Node.js模块加载
    
- deno：可用作deno模块
    
- no-modules：像web目标，但不使用ES模块
    

就使用哪种编译器而言，最简单的方法通常是使用Webpack，因为它是最兼容的。

现在让我们来测试一下！我们将使用Express.js启动一个JavaScript后端服务器。在Rust项目根目录下运行以下代码(为了方便起见)：

```
npm init -ynpm i express express-fileupload
```

接下来，我们将在项目根目录中创建一个server.js文件，并插入以下代码：

```js
const express = require('express');  
const fileUpload = require('express-fileupload');  
const cors = require('cors');  
const bodyParser = require('body-parser');  
const morgan = require('morgan');  
const _ = require('lodash');  
const { convert_image } = require('./pkg/wasm_example.js');  
  
const app = express();  
const port = 3030;  
  
app.use(fileUpload({  
    createParentPath: true  
}));  
  
app.use(cors());  
app.use(bodyParser.json());  
app.use(bodyParser.urlencoded({extended: true}));  
app.use(morgan('dev'));  
  
app.get('/', (req, res) => {  
  res.send(`  
    <h2>With <code>"express"</code> npm package</h2>  
    <form action="/api/upload" enctype="multipart/form-data" method="post">  
      <div>Text field title: <input type="text" name="title" /></div>  
      <div>File: <input type="file" name="file"/></div>  
      <input type="submit" value="Upload" />  
    </form>  
  `);  
});  
  
app.post('/api/upload', (req, res, next) => {  
    const image = convert_image(req.files.file.data);  
  
    res.setHeader('Content-disposition', 'attachment; filename="meme.jpeg"');  
    res.setHeader('Content-type', 'image/jpg');  
    res.send(image);  
});  
  
app.listen(port, () => {  
  console.log(`Example app listening on port ${port}`)  
})
```

这段代码做了以下工作：

- 在端口3030设置了一个Express服务器
    
- 我们在/上有一个路由，当我们在浏览器中访问它时，它会给我们一个HTML表单
    
- 我们有一个API路由，它将从文件上传中获取数据，将其转换为新格式，设置正确的头信息并返回新图像。
    

使用以下命令启动服务器：

```sh
node server.js
```
然后在浏览器中输入http://localhost:3030/，如图：
![](../learning/src/objInfo/assets/Pasted%20image%2020240315225348.png)

请注意，根据用于图像文件格式转换的设置，转换后的文件大小可能会增加。这是因为使用的是无损转换。如果你想使用有损转换来减小文件大小，需要在Rust代码中实例化图像编码器时使用new_with_quality方法。

## 使用其他CLI构建

虽然wasm-bindgen-cli很有用，但它也是我们选项中最底层的CLI，在使用它时可能会遇到一些问题，比如wasm-bindgen版本不兼容问题。让我们快速浏览一下其他CLI，并比较它们之间的异同。

### Wasm-pack

wasm-pack是一个旨在将Rust编译为WASM的一站式工具。它包含一个CLI，可以使用如下命令来安装它：
```sh
cargo install wasm-pack
```
与使用wasm-bindgen-cli相比，它有许多质量上的升级：

- 附带一个WebAssembly分配器wee_alloc，它的代码占用(预压缩)为1kB。
    
- 附带一个panic钩子，可以在浏览器中调试Rust panic消息。
    

要初始化我们的项目，可以使用以下命令：
```sh
wasm-pack new wasm-example-2
```
它将为我们做所有的事情。在代码方面，我们的主函数(和C/JS绑定)将保持与wasm-pack相同，它主要提供工具添加，使编译更容易。

然后使用如下命令构建WASM：

```sh
wasm-pack build  
# 或  
wasm-pack build --target nodejs
```

### napi-rs

napi-rs是一个框架，用于在Rust中构建预编译的Node.js插件。如果你发现使用wasm-bindgen太复杂而无法使用，并且只想编写Node.js的东西，那么这是一个很好的选择。要使用它。可以使用如下命令来安装它(需要npm或它的替代品)：

```
npm install -g @napi-rs/cli
```

安装完成后，就可以使用以下命令来构建新的napi项目了：

```
napi new wasm-example-3
```

napi-rs确实带来了一些代码的变化，你可以在下面看到：我们最终可以摆脱“extern C”块，而是使用napi的bindgen_prelude来包含我们需要的任何东西。

```
use napi::bindgen_prelude::*;use image::io::Reader;use image::ImageFormat;use image::ImageOutputFormat;use std::io::Cursor;#[macro_use]extern crate napi_derive;#[napi]pub fn convert_image(buffer: Buffer) -> Result<Buffer> {    let bytes: Vec<u8> = buffer.into();    let img2 = Reader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();    let mut new_vec: Vec<u8> = Vec::new();    img2.write_to(&mut Cursor::new(&mut new_vec), ImageFormat::Jpeg).unwrap();    Ok(new_vec.into())}
```

这样做的好处是显而易见的：
- 我们不需要使用“extern C”手动导入任何东西
    
- 我们可以轻松地使用Node.js内部组件
    

当然，尽管有这些优点，但是napi-rs只与Node.js兼容，同时还需要使用Node生态系统来更新CLI，从rust优先的角度来看，这是一个有点奇怪的方式。但是，napi-rs是用Rust开始编写Node.js的一种非常简单的方法。如果要为浏览器编写一些通用WASM代码，还是需要使用wasm-pack或wasm-bindgen。

