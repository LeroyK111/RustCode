# Release-plz

release-plz创建了一个完全自动化的发布管道，可以轻松的、频繁的发布更改，而不必担心在从终端发布时可能犯的拼写错误或其他细微的手动错误。主要特点：基于常规提交的版本更新。使用git-cliff更新变更日志，默认使用keep a Changelog格式。使用cargo-semver-checks检测API破坏更改。支持Cargo workspaces。不需要配置。发布前可以选择是否执行cargo update。为每个发布的包创建Git标签。包可以发布到任何cargo registry。基于git标签发布GitHub/Gitea版本

github链接：https://github.com/MarcoIeni/release-plz