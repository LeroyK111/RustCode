# 区块链

```
Substrate 是一个功能齐全的区块链框架，它非常强大。但它也很复杂，难以深入理解，所以我开设这个系列，帮助那些想要深入了解区块链内部的开发者。

Rust 无疑是编写区块链的首选，它也是我最喜欢的编程语言。如果你还没有接触过它，你应该去试试。
```

## 区块

什么是区块？它是区块链的基本单位。在我们的例子中，它只是一个 Rust 结构体。我们可以这样定义它：
```rust
struct Block {
header: BlockHeader,
body: BlockBody,
}
```
区块由两部分组成：BlockHeader 和 BlockBody。每种类型的定义如下：

BlockHeader
```rust
struct BlockHeader {
hash: String,
height: u64,
prev_hash: String,
timestamp: u64,
}
```
BlockBody

`type BlockBody = Vec<String>;`
区块的 Body 部分是一个普通的字符串向量，而头部看起来更有趣。在所有的字段中，prev_hash 是最有趣的，它存储了前一个区块的哈希字段值，我们将在这篇文章后面的链部分讨论它。

height 字段表示这个区块的序列号，新的区块被添加到区块链中时，高度会递增。

timestamp 字段表示创建这个区块时的 Unix 时间戳。所以它与你正在使用的本地机器有关。

而 hash 字段存储了这个区块的哈希值。我们会问：如何计算这个区块的哈希值？因为哈希字段是这个结构体的一部分，所以简单地序列化这个结构体是行不通的。我们需要在做计算时从其他字段中排除这个字段。所以算法看起来像这样：

```rust
    fn calc_block_hash(height: u64, prev_hash: &str, timestamp: u64, body: &Vec<String>) -> String {
        let concated_str = vec![
            height.to_string(),
            prev_hash.to_string(),
            timestamp.to_string(),
            body.concat(),
        ]
        .concat();

        let mut hasher = Sha256::new();
        hasher.update(concated_str.as_bytes());
        hex::encode(hasher.finalize().as_slice())
    }
```

我们不会教授如何编写 Rust 代码的细节，相反，我们主要会描述如何设计它的思路流程。

在这里，我们按照 height, prev_hash, timestamp, body 的顺序连接这个区块的元素。由于 body 是一个向量，我们应该首先连接它。一旦字符串连接完成，我们使用 Sha256 对其进行哈希计算。这一步创建了一个 32 字节的 u8 数组：[u8; 32]。然后我们使用 hex 将其编码为长度为 64 的字符串，这代表了这个区块的哈希。

你会注意到，我们将 prev_hash 值作为这个区块的哈希的来源之一。这非常重要，你可能会想知道我们为什么要这么做。

我们可以按照以下方式测试这个算法：

```rust
#[test]
fn test_block_hash() {
    let block1 = Block::new(10, "aaabbbcccdddeeefff".to_string(), vec![]);
    let block2 = Block::new(10, "aaabbbcccdddeeefff".to_string(), vec![]);
    assert_eq!(block1.header.height, block2.header.height);
    assert_eq!(block1.header.prev_hash, block2.header.prev_hash);
    // XXX: have little probability to fail
    assert_eq!(block1.header.timestamp, block2.header.timestamp);
    // XXX: have little probability to fail
    assert_eq!(block1.header.hash, block2.header.hash);

    assert_eq!(block1.body, block2.body);
}
```

## 链
什么是链？你可以想象一条项链或者一条铁链。在我们的例子中，链是一个抽象的概念，每个区块都存储了前一个区块的哈希字段值。就这样，你看，没有复杂的地方。

在这个链中，每个区块只关心前一个区块，而不关心其他区块。所以它是一个相对简单的结构。

但我们即将遇到一个问题：第一个区块怎么办？它之前没有区块。

是的，对于这个边缘情况，我们需要为 hash 字段设置一个预定义的值。由于这个特殊情况，区块链的第一个区块通常被称为 创世(Genesis) 区块。

![](Pasted%20image%2020240122202750.png)

随着时间的推移，这个结构将无限扩展（或增长）。


## 区块链管理器
我们需要一个管理器来管理区块链。现在它非常简单，只包含一个区块的向量。

```rust
#[derive(Debug)]struct BlockChain {    blocks: Vec<Block>,}
```

并在其上实现一些方法：

```rust
impl BlockChain {    fn new() -> Self {        BlockChain { blocks: vec![] }    }    fn genesis() -> Block {        let txs = vec!["The big brother is watching you.".to_string()];        Block::new(0, "1984, George Orwell".to_string(), txs)    }    fn add_block(&mut self, block: Block) {        self.blocks.push(block);    }}
```

现在我们可以使用这个管理器来构建一个区块链：

```rust
fn main() {    let mut blockchain = BlockChain::new();    let genesis_block = BlockChain::genesis();    let prev_hash = genesis_block.header.hash.clone();    blockchain.add_block(genesis_block);    let b1 = Block::new(1, prev_hash, vec![]);    let prev_hash = b1.header.hash.clone();    blockchain.add_block(b1);    let b2 = Block::new(2, prev_hash, vec![]);    let prev_hash = b2.header.hash.clone();    blockchain.add_block(b2);    let b3 = Block::new(3, prev_hash, vec![]);    let prev_hash = b3.header.hash.clone();    blockchain.add_block(b3);    let b4 = Block::new(4, prev_hash, vec![]);    let prev_hash = b4.header.hash.clone();    blockchain.add_block(b4);    let b5 = Block::new(5, prev_hash, vec![]);    // let prev_hash = b5.header.hash.clone();    blockchain.add_block(b5);    println!("{:#?}", blockchain);}
```

它将打印出来类似下面的东西：

```sh
mike@alberta:~/works/blockchainworks/vintage$ cargo run   Compiling vintage v0.1.0 (/home/mike/works/blockchainworks/vintage)    Finished dev [unoptimized + debuginfo] target(s) in 0.22s     Running `target/debug/vintage`BlockChain {    blocks: [        Block {            header: BlockHeader {                hash: "96cf34aa91e070ddf95eb9e0e8616b24e2f326c80d5fa9746e8dd8f0bec730d6",                height: 0,                prev_hash: "1984, George Orwell",                timestamp: 1705649594,            },            body: [                "The big brother is watching you.",            ],        },        Block {            header: BlockHeader {                hash: "0dd52ac54a9d621c47688f7920cd9eaee18ffe0cca3c83e124b8f78cef8999e5",                height: 1,                prev_hash: "96cf34aa91e070ddf95eb9e0e8616b24e2f326c80d5fa9746e8dd8f0bec730d6",                timestamp: 1705649594,            },            body: [],        },        Block {            header: BlockHeader {                hash: "61e95ab151cfa41c2a74cb076c33511ddf71f45dab0571f5f2db89df7ebc64cf",                height: 2,                prev_hash: "0dd52ac54a9d621c47688f7920cd9eaee18ffe0cca3c83e124b8f78cef8999e5",                timestamp: 1705649594,            },            body: [],        },        Block {            header: BlockHeader {                hash: "dde009c56c1b02d41fec8271e5f990e9b33c84a2cf044de6fc33e96605f90458",                height: 3,                prev_hash: "61e95ab151cfa41c2a74cb076c33511ddf71f45dab0571f5f2db89df7ebc64cf",                timestamp: 1705649594,            },            body: [],        },        Block {            header: BlockHeader {                hash: "f8cd3ab5f6ccc864515635878498e2e26b63b4fbf4dbc60ea3649e859b4a7d27",                height: 4,                prev_hash: "dde009c56c1b02d41fec8271e5f990e9b33c84a2cf044de6fc33e96605f90458",                timestamp: 1705649594,            },            body: [],        },        Block {            header: BlockHeader {                hash: "4415f4993729459f5ff07c7c963890f1d9210d5241f5203b9179c8d3db6e9dac",                height: 5,                prev_hash: "f8cd3ab5f6ccc864515635878498e2e26b63b4fbf4dbc60ea3649e859b4a7d27",                timestamp: 1705649594,            },            body: [],        },    ],}
```

我们做到了，它已经是一个区块链了。
## 如何持久化
到目前为止，我们只是将区块链保存在计算机的内存中，所以如果我们现在关闭计算机，区块链将一无所有。我们最好将整个链存储在我们的计算机上。

一般来说，人们会使用键值数据库（kv db）来存储区块链。为什么使用 kv db 而不是文件或 SQL db 呢？因为它简单且高效。

在我们的例子中，我们将使用 redb 作为我们的存储后端。根据其官方网站，Redb 是一个简单、便携、高性能、ACID、嵌入式键值存储，完全用 Rust 编写，并受到 lmdb 的启发。

接下来，我们需要设计一个存储模式。有以下几点：

- 使用两个表：`blocks` 表用于开发/生产模式，`blocks_fortest` 表用于测试模式；
    
- 每个区块都作为 redb 中的一个键值元素存储，其中键是区块的哈希字段值，值是区块的完全序列化字符串。
    
- 我们需要一个指针指向最后一个区块。'指向'实际上意味着持有区块的哈希值。我们可以使用这个指针从数据库中重构整个链（内存表示）。
    
- 我们还保持了 `height` 和区块的 `hash` 之间的映射关系。
    

然后我们需要将一个 db 实例注入到 BlockChain 管理器结构中。

```rust
#[derive(Debug)]struct BlockChain {    blocks: Vec<Block>,    db: Db,}
```

基于这个管理器实例，我们可以按照以下方式实现一个持久化方法：

```rust
   fn persist_block_to_table(
        &mut self,
        table: TableDefinition<&str, &str>,
        block: &Block,
    ) -> Result<()> {
        let height = &block.header.height;
        let hash = &block.header.hash;
        let content = serde_json::to_string(&block)?;

        // store hash->block pair
        self.db.write_block_table(table, &hash, &content)?;
        // store height->hash pair
        self.db
            .write_block_table(table, &height.to_string(), &hash)?;
        // store the lbp->hash pair (last block pointer to hash)
        self.db
            .write_block_table(table, LAST_BLOCK_POINTER, &hash)?;

        Ok(())
    }
```

其中，我们使用 `serde` 框架并使用 `serde_json` 将整个 `block` 结构体序列化为字符串（json 格式）。如你所见，我们存储了 3 对键值对：

- hash -> 序列化的区块字符串
    
- height -> 区块哈希
    
- lbp (最后一个区块的指针) -> 区块哈希
    

我们可以像这样从数据库中检索一个 `Block`：

```rust
    fn retrieve_block_by_hash_from_table(
        &self,
        table: TableDefinition<&str, &str>,
        hash: &str,
    ) -> Result<Option<Block>> {
        let content = self.db.read_block_table(table, hash)?;
        info!("{:?}", content);
        if let Some(content) = content {
            let b: Block = serde_json::from_str(&content)?;
            Ok(Some(b))
        } else {
            Ok(None)
        }
    }
```
我们使用 `serde_json::from_str()` 来反序列化原始字符串。

接下来，我们需要弄清楚如何从数据库中重新构建一个正确的区块链。我们可以使用一个迭代来做这个，首先获取最后一个区块，然后获取最后一个区块的前一个区块，依此类推。我们可以看看代码：
```rust
    fn populate_from_db_table(&mut self, table: TableDefinition<&str, &str>) -> Result<()> {
        // find last block hash from db
        let last_block_hash = self.db.read_block_table(table, LAST_BLOCK_POINTER)?;
        if last_block_hash.is_none() {
            return Ok(());
        }
        let last_block_hash = last_block_hash.unwrap();

        // retrieve last block
        let block = self.retrieve_block_by_hash_from_table(table, &last_block_hash)?;
        if block.is_none() {
            return Ok(());
        }
        let block = block.unwrap();
        let mut prev_hash = block.header.prev_hash.clone();

        let mut blocks: Vec<Block> = vec![block];
        // iterate to old blockes by prev_hash
        while prev_hash != GENESIS_PREV_HASH {
            let block = self.retrieve_block_by_hash_from_table(table, &prev_hash)?;
            if block.is_none() {
                return Ok(());
            }
            let block = block.unwrap();
            prev_hash = block.header.prev_hash.clone();

            blocks.insert(0, block);
        }

        // contruct an instance of blockchain
        self.blocks = blocks;

        Ok(())
    }
```
我们可以像这样使用这个 API：
```rust
    let mut blockchain = BlockChain::new();
    blockchain
        .populate_from_db()
        .expect("error when populate from db");
```
```rust
#[test]
fn test_store_block_and_restore_block() {
    let mut blockchain = BlockChain::new_to_table(TABLE_BLOCKS_FORTEST);

    // initialization
    let genesis_block = BlockChain::genesis();
    let prev_hash = genesis_block.header.hash.clone();
    blockchain.add_block_to_table(TABLE_BLOCKS_FORTEST, genesis_block);

    let b1 = Block::new(1, prev_hash, vec![]);
    let prev_hash = b1.header.hash.clone();
    blockchain.add_block_to_table(TABLE_BLOCKS_FORTEST, b1);

    let b2 = Block::new(2, prev_hash, vec![]);
    blockchain.add_block_to_table(TABLE_BLOCKS_FORTEST, b2);

    let block_vec = blockchain.blocks.clone();

    blockchain
        .populate_from_db_table(TABLE_BLOCKS_FORTEST)
        .expect("error when populate from db");

    _ = blockchain.db.drop_table(TABLE_BLOCKS_FORTEST);

    for (i, block) in block_vec.into_iter().enumerate() {
        let block_tmp = blockchain.blocks[i].clone();
        assert_eq!(block, block_tmp);
    }
}
```
在我们的代码中，使用 `anyhow` 来帮助管理各种错误，感谢这个漂亮的 crate，它使我们的生活更加轻松。


到目前为止，我们的区块链已经具有了持久化的能力，不再担心会丢失数据。我们已经到达了伟大征程的第一个里程碑。