/*
在Rust中，Duration类型用于操作日期，它表示固定的秒数和纳秒数。Duration有weeks、days、hours的构造函数，但它没有month的构造函数，因为它是一个不能用秒表示的相对值。因此，如果希望以更易于人类阅读的单位来操作日期，例如增加一个月或一年，可以使用这个Chronoutil crate提供的工具。
*/

let delta = RelativeDuration::months(1) + RelativeDuration::days(1);
assert_eq!(
    NaiveDate::from_ymd(2021, 1, 28) + delta,
    NaiveDate::from_ymd(2021, 3, 1)
);
assert_eq!(
    NaiveDate::from_ymd(2020, 1, 28) + delta,
    NaiveDate::from_ymd(2020, 2, 29)
);