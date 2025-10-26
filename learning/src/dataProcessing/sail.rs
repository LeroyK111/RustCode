/*


sail - 流处理引擎，完全兼容PySpark，比Spark快4倍，硬件消耗低 94%Sail的使命是统一流处理、批处理和计算密集型（AI）工作负载。目前，Sail在单进程环境下提供了Spark SQL和Spark DataFrame API的即插即用替代方案。补充解释："流处理"指的是实时处理持续输入的数据。"批处理"是指处理一批已收集的数据。"计算密集型工作负载"通常指需要大量计算资源的任务，如AI/机器学习任务。"即插即用替代方案"意味着用户可以轻松地用Sail替换现有的Spark SQL和DataFrame API，而无需大幅修改代码。"单进程环境"表示Sail目前专注于在单个计算进程中运行，而不是分布式系统。https://github.com/lakehq/sail
*/