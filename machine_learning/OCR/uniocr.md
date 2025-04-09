`uniOCR` 是一个用 Rust 编写的通用 OCR 引擎，旨在提供跨平台的 OCR 解决方案，支持多种 OCR 提供者，包括 macOS 原生 Vision Kit API、Windows OCR 引擎、Tesseract OCR 以及自定义的云服务提供商。以下是对该仓库的详细介绍：

**主要特性**

1. **多平台支持**：支持 macOS、Windows 和 Linux 等主流操作系统，针对不同平台提供原生的 OCR 解决方案。
    
2. **多 OCR 提供者集成**：
    

- **macOS**：使用原生的 Vision Kit API 进行 OCR 识别。
    
- **Windows**：利用 Windows OCR 引擎进行识别。
    
- **Tesseract**：支持 Tesseract OCR，并可使用自定义模型，具备快速初始化和缓存功能。
    
- **自定义云服务**：支持自定义的 OCR 提供者，可通过 API 进行识别。
    

4. **统一 API 接口**：提供单一的接口来调用不同的 OCR 提供者，方便切换和使用，同时支持批量处理。
    
5. **高性能设计**
    

- 采用 async/await 机制，支持并行处理。
    
- 注重内存效率，对不安全代码的内存泄漏进行了严格测试。
    

Github 仓库：https://github.com/mediar-ai/uniOCR