# yank

从 `stdin` 读取输入并显示一个选择界面，该界面允许选择一个字段并将其复制到剪贴板。

## Usage
- `sudo dmesg | yank` : 使用默认分隔符（\f, \n, \r, \s, \t）
- `sudo dmesg | yank -l` : 输入单行
- `echo hello=world | yank -d =` : 使用特定分 `=` 隔符输入
- `ps ux | yank -g "[0-9]+"` : 只有与特定正则表达式匹配的内容才输入

## Links
- More information: https://manned.org/yank
