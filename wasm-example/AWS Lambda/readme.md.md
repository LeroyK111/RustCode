### 用于AWS Lambda的超小型、极简化的自定义运行时

这是一个用于AWS Lambda的超小型、极简化的自定义运行时,提供了C和Rust的API绑定。它的特点包括:

1. 没有抽象膨胀,HTTP头和JSON载荷以原始char*缓冲区的形式传递。
    
2. 除了标准C库外没有其他依赖,可选择需要的功能。
    
3. 动态链接到glibc,以最小化二进制大小。
    
4. 无需分配器依赖,可自带arena或gc分配器,或使用malloc。
    
5. 提供Rust语言绑定,无需std、main和tokio。
    
6. 极小的部署包大小(~5kb压缩)。
    
7. 非常快的冷启动时间(4-5毫秒)。
    
8. 最小化计算资源浪费,减缓全球变暖。
    

它是C语言AWS Lambda运行时的替代品,也是Rust语言AWS Lambda运行时的替代品。

https://github.com/refacktor-aws/aws-lambda-libc-runtime