# zgrep

在压缩文件中从文件中 grep 文本模式（等价于 `grep -Z`）。

## Usage
- `zgrep 模式 路径/到/压缩/文件` : 在压缩文件中 grep 一个模式（区分大小写）
- `zgrep -i 模式 路径/到/压缩/文件` : 在压缩文件中 grep 一个模式（不区分大小写）
- `zgrep -c 模式 路径/到/压缩/文件` : 输出包含匹配模式的行数
- `zgrep -v 模式 路径/到/压缩/文件` : 显示不包含该模式的行（反转搜索功能）
- `zgrep -e "模式_1" -e "模式_2" 路径/到/压缩/文件` : 在压缩文件中为多个模式执行 grep
- `zgrep -E 正则表达式 路径/到/文件` : 使用扩展的正则表达式（支持 `?`、`+`、`{}`、`()` 和 `|`）
- `zgrep -C|B|A 3 模式 路径/到/压缩/文件` : 打印每个匹配项前后各 3 行的上下文（Content）、Before 或 After

## Links
- More information: https://manned.org/zgrep
