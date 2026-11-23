# hx

Helix，一个后现代的文本编辑器，为不同类型的文本操作提供了几种模式。 按 `<i>` 进入插入模式。按 `<Esc>` 进入正常模式，并且可以使用 Helix 命令。

## Usage
- `hx 路径/到/文件` : 打开文件
- `hx --vsplit 路径/到/文件1 路径/到/文件2` : 并排打开多个文件
- `hx --tutor` : 显示 Helix 教程（也可以在 Helix 中按 `<Esc>` 后输入 `<:>tutor<Enter>` 访问）
- `<:>theme 主题名称<Enter>` : 更改 Helix 主题
- `<:>wq<Enter>` : 保存并退出
- `<:>q!<Enter>` : 强制退出并不保存
- `<u>` : 撤销上次操作
- `</>搜索模式<Enter>` : 搜索文件中的关键字（按 `<n>`/`<N>` 前往下一个/上一个匹配）

## Links
- More information: https://manned.org/man/debian-forky/hx
