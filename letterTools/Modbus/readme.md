### modbus-relay - 高性能Modbus TCP到RTU中继程序

modbus-relay是一个用Rust语言编写的高性能Modbus TCP到RTU的中继程序,名为ModbusRelay。该项目起源于作者在开发家庭通风系统控制软件时的个人需求,由于树莓派3B的内存只有1GB,直接在上面开发不太实际,因此需要一个轻量级的桥接器在树莓派上运行,用于连接Modbus TCP和RTU,同时允许在更强大的硬件上开发和部署主控制软件。

该程序的主要特点包括:

- 使用异步I/O(Tokio)实现最大性能
    
- 零拷贝操作以高效利用内存
    
- 智能RS485处理,可配置RTS控制
    
- 内置HTTP监控API
    
- 全面的错误处理和连接管理
    

技术栈包括Rust(使用Tokio异步运行时)、Axum作为HTTP服务器,以及各种Rust crate用于配置和日志记录。

https://github.com/aljen/modbus-relay