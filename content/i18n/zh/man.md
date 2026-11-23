# man

展示手册分页 (manual page)。

## Usage
- `man command` : 展示一条命令的使用手册分页
- `man -Hbrowser_name command` : 在浏览器中打开一条命令的使用手册分页 (`$BROWSER` 环境变量可以替换 `=browser_name`)
- `man 7 command` : 从第七章节展示一条命令的使用手册分页
- `man -f command` : 列表展示一条命令的所有可用章节
- `man -w` : 展示搜索手册分页的路径
- `man -w command` : 展示使用手册分页的位置, 而不是手册分页本身
- `man -L locale command` : 使用特定的语言来展示使用手册分页
- `man -k "search_string"` : 搜索包含搜索字符串的手册页

## Links
- More information: https://manned.org/man
