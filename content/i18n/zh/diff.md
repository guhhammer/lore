# diff

比较文件或目录。

## Usage
- `diff 原文件 新文件` : 比较文件 （列出从`原文件`到`新文件`的修改）
- `diff -w 原文件 新文件` : 比较文件，忽视空白字符
- `diff -y 原文件 新文件` : 比较文件，并排展示文件差异
- `diff -u 源文件 新文件` : 比较文件，以统一形式展示不同区别 （ 形式同 `git diff`）
- `diff -r old_directory new_directory` : 递归地比较目录 （展示不同文件和目录的名字和差异）
- `diff -r -q old_directory new_directory` : 比较目录（只展示不同文件的文件名）
- `diff -a -u -N 原文件 新文件 >diff.patch` : 根据两个文本文件的差异，为 Git 创建补丁文件，不存在的文件会被视为空文件
- `diff -d --color=always 原文件 新文件` : 比较文件，打印彩色的输出，并尽可能找出最小的改变

## Links
- More information: https://manned.org/diff
