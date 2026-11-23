# git bugreport

收集系统和用户的调试信息，生成文本文件以帮助报告 Git 中的问题。

## Usage
- `git bugreport` : 在当前目录创建新的问题报告文件
- `git bugreport -o path/to/directory` : 在指定目录创建新的问题报告文件（如目录不存在则自动创建）
- `git bugreport -s %m%d%y` : 使用指定格式的时间戳后缀创建问题报告文件（ `strftime` 格式）

## Links
- More information: https://git-scm.com/docs/git-bugreport
