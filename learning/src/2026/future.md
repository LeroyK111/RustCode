Rust 团队正式发布 1.96.0 稳定版（2026-05-28），带来多项值得关注的新特性。

新 Range* 类型（core::range） - core::range::Range、RangeFrom、RangeInclusive 及配套迭代器稳定化 - 新类型实现 IntoIterator 而非直接实现 Iterator，因此可以同时实现 Copy，彻底解决了旧 Range 类型无法同时为 Copy 的历史问题 - 库作者建议在公共 API 使用 impl RangeBounds，同时兼容旧版和新版 Range

新宏 assert_matches! / debug_assert_matches! - 检查值是否匹配指定模式，失败时打印 Debug 信息，比 assert!(matches!(..)) 诊断信息更丰富 - 因与第三方同名 crate 冲突，未加入 prelude，需显式 use core::assert_matches; 导入

Cargo 安全修复 - CVE-2026-5223（中危）：修复 crate tarball 中 symlink 处理，防止恶意 crate 覆盖同注册表其他 crate 缓存 - CVE-2026-5222（低危）：修复标准化 URL 认证问题 - crates.io 用户不受影响