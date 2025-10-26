# TinyUFO - 无锁高性能缓存
TinyUFO 是 Cloudflare 开源的 Pingora 中的一个组件，结合了最先进的 S3-FIFO 算法，利用 TinyLFU 作为准入策略，相较于 LRU 和 Moka 在 zipf = 1 的情况下，提供了更高的命中率。

此外，由于 TinyUFO 使用无锁数据结构，在性能上远远超过 Lru 和 Moka ，特别是在混合读写的工作负载下。