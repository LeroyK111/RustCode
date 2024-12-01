/*
Crossbeam crate为并发编程提供了一组工具：原子、数据结构、内存管理、线程同步等等。例如，与std channels相比，Crossbeam中channels的实现性能更高，并且允许多个生产者和多个消费者(multi-producer multi-consumer)，而不像std channels只允许单个消费者(multi-producer single-consumer)。
*/