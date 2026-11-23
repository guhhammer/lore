# fzf

命令行模糊查找器。

## Usage
- `find 路径/到/目录 -type f | fzf` : 对指定目录中的所有文件启动 `fzf`
- `ps aux | fzf` : 为正在运行的进程启动 `fzf`
- `find 路径/到/目录 -type f | fzf -m >路径/到/文件` : 使用 `<Shift Tab>` 选择多个文件并将结果写入文件
- `fzf -q "查询词"` : 使用指定查询词启动 `fzf`
- `fzf -q "^core go$ | rb$ | py$"` : 对以 core 开头、以 go, rb 或 py 结尾的条目启动 `fzf`
- `fzf -q '!pyc travis'` : 对不匹配 pyc 且完全匹配 travis 的条目启动 `fzf`

## Links
- More information: https://github.com/junegunn/fzf#usage
