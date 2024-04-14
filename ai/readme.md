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

## aichat - Rust AI集成器

一个统一的命令行界面，集成了 10 多个平台：

OpenAI: GPT3.5/GPT4 (paid, vision)
Azure-OpenAI (paid)
OpenAI-Compatible platforms
Gemini: Gemini-1.0/Gemini-1.5 (free, vision)
VertexAI (paid, vision)
Claude: Claude3 (vision, paid)
Mistral (paid)
Cohere (paid)
Ollama (free, local)
Ernie (paid)
Qianwen (paid, vision)
Moonshot (paid)
Github https://github.com/sigoden/aichat


## C2PA使用Rust来实现其目标
C2PA（内容来源和真实性联盟）是一个开放标准，旨在帮助用户识别数字文件（例如图像、录音或视频）的创建者、内容以及其编辑方式。其目标是使用户能够更轻松地将真实照片或视频与人工智能生成的图像和数字艺术区分开来。

C2PA Rust 库实现了 C2PA 的一个子集，用来做为一个标准参考。

Repo: https://opensource.contentauthenticity.org/docs/rust- sdk/


## Candle Tensor 技术

最知名的 Tensor 类型是 PyTorch 机器学习框架中的 Tensor。但对于 Rust，我们也有 Tensor 类型。在 Rust 社区中最受欢迎的深度学习框架：Candle、Burn、Dfdx 都为我们提供了自己的 Tensor 类型定义。

其中，由 AI 明星创业公司 HuggingFace 开发的 Candle，是与 PyTorch 最相似的框架；在本文的后续部分，我将阐述其中的 Tensor 的使用。


## Rust ML框架

1，Candle

Candle是一个为简单和高性能而设计的深度学习框架。它为定义和训练神经网络提供了一个简约的API，利用了基于内核并行计算的强大功能。Candle依赖底层的cuTENSOR和cuDNNv8库，在NVIDIA gpu上实现高效执行。

2，Burn

Burn的目标是在Rust中构建一个成熟的机器学习栈。它包含各种组件，包括数据加载、模型定义、训练、超参数优化等。Burn使用自定义内核代码进行计算，从而更好地控制底层操作。

3，DFDX

DFDX是Rust中用于深度学习的可微分编程库。它采用了一种独特的方法，为使用声明式、函数式编程风格进行构建和训练模型提供了一个框架。DFDX利用自动微分，并通过CUDA后端提供对GPU加速的支持。

4，tch-rs

tch-rs是流行的PyTorch深度学习库的Rust绑定。它为PyTorch提供了一个安全和习惯的Rust接口，允许开发人员在编写高性能Rust代码时利用广泛的PyTorch生态系统。tch-rs促进了Rust和Python在ML应用程序之间的互操作性。


如何选择合适的框架？

在选择Rust ML框架时，请考虑以下因素：

1，项目需求

评估项目的具体需求，如深度学习、经典ML算法，或两者的结合。Candle和tch-rs主要专注于深度学习，而Burn和DFDX则为各种ML技术提供更广泛的支持。

2，性能和效率

如果计算效率是一个关键要求，与更全面的框架(如Burn或可与python互操作的tch-rs)相比，Candle的专业深度学习方法和DFDX的自动微分提供了更好的性能。

3，灵活性和自定义

Burn和DFDX为机器学习管道提供了更大的灵活性和控制，允许自定义和扩展。Candle和tch-rs虽然功能强大，但由于它们依赖于外部库，因此在定制方面可能会受到更多限制。

4，生态系统和社区支持

这四个框架都在积极开发。tch-rs受益于广泛的PyTorch生态系统和社区支持；Burn和DFDX也拥有不断增长的社区；而Candle的专注范围可能会导致更小但更专注的用户群。

5，学习曲线和熟悉度

如果你或你的团队有PyTorch的使用经验，tch-rs可能会有一个更流畅的学习曲线。另外，如果更熟悉函数式编程范例，DFDX的声明式方法可能会很有吸引力。Candle和Burn提供了更传统的命令式编程风格。

6，集成要求

如果你的项目涉及到与现有的Rust代码库集成或利用其他Rust库，Burn、DFDX和tch-rs可能会提供更好的集成能力，因为它们有更广泛的作用域或与Python的互操作性。

7，部署和生产注意事项

评估ML应用程序的部署需求，例如对容器化、无服务器环境或嵌入式系统的需求。像Candle和Burn这样的框架，作为纯Rust解决方案，可能在某些部署场景中更有优势。
