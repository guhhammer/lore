# locate

快速查找文件名。

## Usage
- `locate "关键字"` : 在数据库中查找关键字。注意：数据库定期重新更新（通常每周或每天）
- `locate */文件名` : 按文件名查找文件（不包含填充字符的模式被解释为 `*关键字*`）
- `sudo /usr/libexec/locate.updatedb` : 重新建立文件数据索引数据库。如果要查找最近添加的文件，则需要执行此操作

## Links
- More information: https://keith.github.io/xcode-man-pages/locate.1.html
