// Humantime crate 以人类可读的格式为std::time::{Duration, SystemTime}提供了格式化器和解析器。它还通过humantime-serde crate与serde集成。例如，可以在应用程序/服务配置中以可读格式指定Duration值，而不是在变量名中使用度量单位，从而减少错误的可能性：
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[test]
fn format() {
    let duration = Duration::new(9420, 0);
    let as_str = "2h 37m";
    assert_eq!(humantime::format_duration(duration).to_string(), as_str);
    assert_eq!(humantime::parse_duration(as_str), Ok(duration));
}

#[derive(Serialize, Deserialize)]
struct Foo {
    #[serde(with = "humantime_serde")]
    timeout: Duration,
}

#[test]
fn serde() {
    let input = r#" { "timeout": "3 days 1hour 12min 5s" } "#;
    let foo: Foo = serde_json::from_str(input).unwrap();
    assert_eq!(foo.timeout, Duration::new(263525, 0));
}
