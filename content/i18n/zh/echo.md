# echo

输出给定参数。

## Usage
- `echo "Hello World"` : 输出文本信息. 注意： 引号是可选的
- `echo "My path is $PATH"` : 输出带有环境变量的信息
- `echo -n "Hello World"` : 打印不带尾随换行符的信息
- `echo "Hello World" >>file.txt` : 向文件添加信息
- `echo -e "Column 1\tColumn 2"` : 启用反斜杠转义的解释（特殊字符）
- `echo $?` : 输出执行的上一条指令的退出状态（注：在 Windows 命令提示符和 PowerShell 中，对应的命令分别是 `echo %errorlevel%` 和 `$lastexitcode`）

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
