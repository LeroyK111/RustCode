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