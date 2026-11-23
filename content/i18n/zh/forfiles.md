# forfiles

选择一个或多个文件以执行指定的命令。

## Usage
- `forfiles` : 在当前的目录中寻找文件
- `forfiles /p 目录的路径` : 在一个指定目录中寻找文件
- `forfiles /c "命令"` : 为每个文件执行指定的命令
- `forfiles /m 通配符` : 使用通配符来寻找指定的文件
- `forfiles /s` : 递归寻找文件
- `forfiles /d +5` : 搜索超过 5 天的文件
