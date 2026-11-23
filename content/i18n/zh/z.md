# z

记录被使用次数最多的目录并允许在它们之间以字符串或正则表达式来进行匹配和跳转。

## Usage
- `z foo` : 跳转到一个名字带有 "foo" 的文件夹
- `z foo bar` : 跳转到一个名字带有 "foo" 并且后面带有 "bar" 的文件夹（例：`fooesbar`）
- `z -r foo` : 跳转到名字带有 "foo" 并且拥有最高访问次数的文件夹
- `z -t foo` : 跳转到最近使用的名字带有 "foo" 的文件夹
- `z -l foo` : 列出在 `z` 的数据库中名字带有 "foo" 的文件夹
- `z -x` : 将当前文件夹从 `z` 的数据库中移除
- `z -c foo` : 限制匹配结果为当前目录的子目录

## Links
- More information: https://github.com/rupa/z
