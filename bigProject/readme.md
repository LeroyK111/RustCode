# 如何管理复杂的大型Rust项目

在本文中，将分享组织大型Rust项目的方法。虽然这不是一个权威的指南，但它包含了通过试验验证和错误收集得到的一些建议和见解。Rust的构建系统Cargo遵循约定优于配置的原则。它提供了一组合理的默认值，适用于小型项目，特别是发布在crate.io上的库。虽然这些默认值并非完美无瑕，但它们通常是有效的，有助于形成一致的生态系统。然而，当涉及到作为工作空间组织的大型、多crate项目时，Cargo就不那么规范了。工作区是高度灵活的，Cargo不会为它们强制执行特定的布局。因此，开发人员尝试不同的结构，通常取得不同程度的成功。开门见山地说，对于1万到100万行代码的项目来说，扁平布局是最有意义的。rust-analyzer(20万行)就是一个很好的例子。布局如下：

rust-analyzer/
  Cargo.toml
  Cargo.lock
  crates/
    rust-analyzer/
    hir/
    hir_def/
    hir_ty/
    ...
在项目的根目录，Cargo.toml定义了一个虚拟清单：[workspace]
members = ["crates/*"]


其他所有内容(包括rust-analyzer的“main”crate)都嵌套在crates/下面的一层。每个目录的名称等于crate的名称：[package]
name = "hir_def"
version = "0.0.0"
edition = "2021"

## 扁平比嵌套好有趣的是，这个建议违背了按层次组织的观点：
rust-analyzer/
  Cargo.toml
  src/
  hir/
    Cargo.toml
    src/
    def/
    ty/


在这种情况下，有几个原因可以解释为什么扁平比嵌套好。首先，crates的Cargo-level命名空间是扁平的。不可能在Cargo中编写hir::def。因此，crate的名称中通常有前缀。嵌套的树形布局创建了另一种层次结构，这增加了不一致的可能性。其次，即使是比较大的列表也比小的树更容易理解。命令 ls ./crates 提供了项目的鸟瞰图，这个视图足够小：

base_db
cfg
flycheck
hir
hir_def
hir_expand
hir_ty
ide
ide_assists
ide_completion
ide_db
ide_diagnostics
ide_ssr
limit
mbe
parser
paths
proc_macro_api
proc_macro_srv
proc_macro_test
profile
project_model
rust-analyzer
sourcegen
stdx
syntax
test_utils
text_edit
toolchain
tt
vfs


对基于嵌套树的布局做同样的事情就比较困难了。查看单个分支并不能告诉你哪些文件夹包含嵌套的crates，查看所有分支列出了太多文件夹。
的确，嵌套结构比扁平结构具有更好的伸缩性。

但在你的项目没有超过一百万行代码时，扁平结构更适合，因为项目中的cratea数量只需要一个屏幕就能显示。最后，分层布局的最后一个问题是没有完美的层次结构。对于扁平结构，添加或分割crates是微不足道的。对于树，就需要找出放置新crate的位置，如果没有完美匹配的crates，你必须：在顶部添加一个愚蠢的几乎是空的文件夹添加一个捕获所有utils文件夹将代码放在已知的次优目录中。对于长期的多人项目来说，这是一个重要的问题——树形结构往往会随着时间的推移而恶化，而扁平结构则不需要维护。

# 小技巧

将工作区的根目录设置为虚拟清单。将主crate放入根目录可能很诱人，但这会用src/污染根目录，需要将--workspace传递给每个Cargo命令，并在本来一致的结构中添加异常。不要屈服于从文件夹名称中去掉通用前缀的诱惑。如果每个crate的名称与它所在的文件夹完全相同，那么导航和重命名就会变得更容易。Cargo.toml反向依赖的同时也涉及到文件夹和crate的名称，当它们完全相同时，这是很有用的。对于大型项目，大量的存储库膨胀通常来自于临时的自动化makefile和各种各样的prepare.sh脚本。为了避免特别工作流的膨胀和扩散，在专用的crate中用Rust编写所有自动化。对此有用的一个模式是使用cargo xtask。对于不打算发布的内部crate，使用version = "0.0.0"。如果确实想要发布具有适当server API的crate子集，请慎重考虑。将所有这些crate提取到单独的顶级文件夹libs/中，因为检查lib/中的东西更容易。有些crates只包含一个文件。对于这些目录，很有可能扁平化src目录并保留lib.rs和Cargo.toml在同一目录下。建议不要这样做，即使crate现在是单个文件，它可能会在以后扩展。