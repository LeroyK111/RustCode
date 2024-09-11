/*

1，高级原语：Jiff的目标是提供高级日期时间原语，这些原语被设计成难以被误用。这种对用户友好设计的关注有助于开发人员在处理日期和时间时避免常见的陷阱。
2，时区集成：Jiff与IANA时区数据库无缝集成，确保准确的时区转换。在Unix系统上，它从/usr/share/zoneinfo读取时区数据，在Windows上，它将时区数据库的副本嵌入到编译库中。
3，性能：在优先考虑易用性的同时，Jiff不会在性能上妥协。它包括对日期时间操作的优化。该库的基准测试表明性能还算合理，不过还有进一步改进的计划。
4，序列化支持：Jiff为Serde提供了可选的支持，允许开发人员轻松地序列化和反序列化datetime对象。此特性对于需要存储或传输日期时间数据的应用程序特别有用。
5，错误处理：该库使用Rust健壮的错误处理机制来提供清晰和可操作的错误消息，帮助开发人员快速识别和解决问题。

*/

use jiff::{Timestamp, ToSpan};

fn main()->Result<(),jiff::Error>{
    let time:Timestamp = "2024-08-30T01:14:00Z".parse()?;
    let zoned = time
    .intz("America/New_York")?
    .checked_add(1.month().hours(2))?;
    assert_eq!(
        zoned.to_string(),
        "2024-09-29T23:14:00-04:00[America/New_York]"
    );
    //或者，如果你想要一个RFC3339格式的字符串
    assert_eq!(zoned.timestamp().to_string(),"2024-09-30T03:14:00Z");
    println!("{}", zoned);
    Ok(())
}