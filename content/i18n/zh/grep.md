# grep

使用 `regex` 式查找文件中的模式。

## Usage
- `grep "模式字符串" 路径/到/文件1 路径/到/文件2` : 在文件中查找模式
- `grep -F "字符串" 路径/到/文件` : 查找精确的字符串（禁用 `regex` 式）
- `grep -rI "模式字符串" 路径/到/目录` : 在目录中递归查找所有文件中的模式，忽略二进制文件
- `grep --context|--before-context|--after-context 3 "模式字符串" 路径/到/文件` : 打印每个匹配项周围、之前或之后的 3 行上下文
- `grep -Hn --color=always "模式字符串" 路径/到/文件` : 打印每个匹配项的文件名和行号，并带彩色输出
- `grep -o "模式字符串" 路径/到/文件` : 仅打印匹配的文本
- `cat 路径/到/文件 | grep -v "模式字符串"` : 从 `stdin` （标准输入）中读取数据，不打印匹配模式的行
- `grep -Ei "模式字符串" 路径/到/文件` : 使用扩展 `regex` 式（支持 `?`、`+`、`{}`、`()` 和 `|`），不区分大小写模式

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
