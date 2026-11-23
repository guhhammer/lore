# exa

`ls` 的现代替代品（列出目录内容）。

## Usage
- `exa -1` : 每行列出一个文件
- `exa -a` : 列出所有文件，包括隐藏文件
- `exa -l -a` : 以长格式列出所有文件（包含权限、所有者、大小和修改日期）
- `exa -r -s size` : 列出文件，最大的排在最上面
- `exa -l -T -L 3` : 显示文件树，深度为三层
- `exa -l -s modified` : 按修改日期排序列出文件（从最旧的开始）
- `exa -l -h --icons --git` : 列出文件及其表头、图标和 Git 状态
- `exa --git-ignore` : 不列出 `.gitignore` 中提到的文件

## Links
- More information: https://github.com/ogham/exa#command-line-options
