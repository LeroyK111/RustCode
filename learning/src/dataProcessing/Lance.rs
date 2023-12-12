/*
Lance是一种柱状数据格式，Lance是Lancedb矢量数据库使用的格式。

Lance的主要特点包括:

高性能随机存取：比Parquet快100倍，不牺牲扫描性能。

矢量搜索：在毫秒内找到最近的邻居，并将olap查询与矢量搜索相结合。

零拷贝，自动版本控制：无需额外基础设施即可管理数据的版本。

生态系统集成：Apache Arrow, Pandas, Polars, DuckDB等。



Lance适合的场景：

建立搜索引擎和功能商店。

需要高性能IO和shuffle的大规模ML训练。

存储、查询和检查深度嵌套的数据，用于机器人或像图像、点云等大数据块。



github地址：
https://github.com/lancedb/lance
*/