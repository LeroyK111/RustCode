# gitoxide 的性能优化
gitoxide 项目中引入了 status() 迭代器，它使得在处理 git 仓库状态时更加高效。此外，项目还实现了无需使用 git2 库的 onefetch 功能，显著提升了性能。

新的 gix-status 库允许并行处理且支持重命名跟踪，而 gix-dir 库则用于遍历目录并找到未跟踪的文件。这些改进使gitoxide能更高效地处理文件状态和目录遍历。

通过增加并行处理和优化 API 的设计，gitoxide 在多个知名的代码仓库中展示了比 git2 更好的性能表现（gitoxide的新功能已被应用到 Cargo 项目中，并提高了API的可用性）