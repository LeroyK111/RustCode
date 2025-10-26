// 和quickcheck一样，protest也是一个基于属性的测试框架。然而，与quickcheck相比，它可以更灵活的生成输入数据，尽管对于复杂的数据，它可能比quickcheck运行时间长得多。
proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        parse_date(&s);
    }

    #[test]
    fn parses_date_back_to_original(y in 0u32..10000,
                                    m in 1u32..13,
                                    d in 1u32..32)
    {
        let result = parse_date(&format!("{:04}-{:02}-{:02}", y, m, d)).unwrap();

        prop_assert_eq!((y, m, d), result);
    }
}