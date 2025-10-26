/*
用于快照测试的库。快照表示测试的预期结果，通常存储在单独的文件中。该库提供了一个命令行实用程序，可以方便地更新快照。其中一个有用的功能是编辑。它允许测试随机或不确定顺序的字段值，例如HashSet：
*/

#[derive(serde::Serialize)]
pub struct User {
    id: Uuid,
    username: String,
    flags: HashSet<&'static str>,
}
#[test]
fn redactions() {
    let user = User {
        id: Uuid::new_v4(),
        username: "john_doe".to_string(),
        flags: maplit::hashset! {"zzz", "foo", "aha"},
    };
    insta::assert_yaml_snapshot!(user, {
        ".id" => "[uuid]",
        ".flags" => insta::sorted_redaction()
    });
}

// 对于这个测试，自动生成快照snapshots/insta__tests__redactions.snap，包含以下内容：