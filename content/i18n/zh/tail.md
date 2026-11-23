# tail

显示文件的最后一部分。

## Usage
- `tail -n n 路径/到/文件` : 显示文件中最后 n 行
- `tail -n +行号 路径/到/文件` : 从特定行号开始打印文件后续内容
- `tail -n 字节数 路径/到/文件` : 从给定文件末尾打印特定字节数量的内容
- `tail -f 路径/到/文件` : 打印给定文件的最后几行并保持读取直到 `<Ctrl c>`
- `tail -F 路径/到/文件` : 保持读取文件直到 `<Ctrl c>`，即使文件无法访问
- `tail -n n -s t -f 路径/到/文件` : 显示文件中最后 n 行并每 t 秒刷新一次

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html
