# Ocrs

ocrs是一个用Rust编写的库和CLI工具，用于从图像中提取文本，称为OCR(光学字符识别)。

它的目标是创建一个现代化的OCR引擎：

与早期的引擎(如Tesseract)相比，它可以很好地处理各种各样的图像(扫描文档，包含文本的照片，截图等)，而无需或更少的预处理工作。这是通过在流水线中更广泛地使用机器学习来实现的。

易于在各种平台(包括WebAssembly)上编译和运行

接受过开放和自由许可数据集的培训

拥有易于理解和修改的代码库