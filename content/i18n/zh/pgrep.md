# pgrep

按名称查找进程。

## Usage
- `pgrep 进程名称` : 查找命令行字符串匹配的进程，并输出其 PID
- `pgrep -l 进程名称` : 以长格式（long output）输出 PID 和进程名称
- `pgrep -f "进程名称 参数"` : 按完整（full）参数列表匹配进程，而不只匹配进程名称
- `pgrep -fl "进程名称"` : 以列表（list）形式输出每个匹配进程的 PID 和完整（full）参数列表
- `pgrep -u 用户名 进程名称` : 查找由指定用户（user，有效 UID）运行的进程
- `pgrep -n 进程名称` : 只输出最新（newest，最近启动）的匹配进程

## Links
- More information: https://keith.github.io/xcode-man-pages/pgrep.1.html
