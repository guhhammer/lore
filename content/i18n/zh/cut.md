# cut

从标准输入或文件中剪切字段。

## Usage
- `命令 | cut --characters|fields 1|1,10|1-10|1-|-10` : 打印每行的特定字符/或属性范围
- `命令 | cut -d "分隔符" -f 1|1,10|1-10|1-|-10` : 打印每行由指定分隔符分割的属性范围
- `cut -c 1|1,10|1-10|1-|-10 路径/到/文件` : 打印文件每行的字符范围
- `命令 | cut -z -f 1` : 打印以 `NUL` 结尾的行的特定字段（例如 `find . -print0`）而不是换行符

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cut-invocation.html
