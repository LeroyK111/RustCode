# AI 序列


## OCR技术

### OCRS

```shell
cargo install ocrs-cli

# 提取文本
ocrs image.png -o content.txt

# 提取文本和布局信息
ocrs image.png --json -o content.json

# 注释图像以显示检测到的单词和线条的位置：
ocrs image.png --png -o annotated.png
```


## Candle Tensor 技术

最知名的 Tensor 类型是 PyTorch 机器学习框架中的 Tensor。但对于 Rust，我们也有 Tensor 类型。在 Rust 社区中最受欢迎的深度学习框架：Candle、Burn、Dfdx 都为我们提供了自己的 Tensor 类型定义。

其中，由 AI 明星创业公司 HuggingFace 开发的 Candle，是与 PyTorch 最相似的框架；在本文的后续部分，我将阐述其中的 Tensor 的使用。