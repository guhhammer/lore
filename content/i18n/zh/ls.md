# ls

列出目录中的内容。

## Usage
- `ls -1` : 列出目录中的文件，每个文件占一行
- `ls -a` : 列出包含隐藏文件的所有文件
- `ls -F` : 列出所有文件，如果是目录，则在目录名后面加上「/」
- `ls -la` : 列出包含隐藏文件的所有文件信息，包括权限，所有者，大小和修改日期
- `ls -lh` : 列出所有文件信息，大小用人类可读的单位表示（KiB, MiB, GiB）
- `ls -lSR` : 列出所有文件信息，按大小降序排序
- `ls -ltr` : 列出所有文件信息，按修改日期从旧到新排序
- `ls -d */` : 只列出目录

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html
