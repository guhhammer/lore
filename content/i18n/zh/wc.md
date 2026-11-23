# wc

计数行、单词或字节。

## Usage
- `wc -l 路径/到/文件` : 计数文件中的行数
- `wc -w 路径/到/文件` : 计数文件中的单词数
- `wc -c 路径/到/文件` : 计数文件中的字节数
- `wc -m 路径/到/文件` : 计数文件中的字符数（考虑所有多字节的字符）
- `find . | wc` : 使用 `stdin`，按顺序计数行、单词和字节
- `wc -L 路径/到/文件` : 计算最长行的长度（字符数）

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html
