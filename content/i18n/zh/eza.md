# eza

基于 `exa` 构建的现代化、持续维护的 `ls` 替代品。

## Usage
- `eza -1` : 每行列出一个文件
- `eza -a` : 列出所有文件，包括隐藏文件
- `eza -al` : 以长格式列出所有文件（包含权限、所有者、大小和修改日期）
- `eza -r -s size` : 列出文件，最大的排在最上面
- `eza -lT -L 3` : 显示文件树，深度为三层
- `eza -l -s modified` : 按修改日期排序列出文件（从最旧的开始）
- `eza -lh --icons --git` : 列出文件及其表头、图标和 Git 状态
- `eza --git-ignore` : 不列出 `.gitignore` 中提到的文件

## Links
- More information: https://github.com/eza-community/eza
