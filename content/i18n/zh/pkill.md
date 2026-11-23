# pkill

按名称向进程发送信号。 通常用于停止进程。

## Usage
- `pkill "进程名称"` : 终止所有匹配指定名称的进程
- `pkill -f "命令名称"` : 根据完整命令而不只是进程名称，终止所有匹配的进程
- `pkill -9 "进程名称"` : 强制终止匹配的进程（该信号不能被捕获或忽略）
- `pkill -USR1 "进程名称"` : 向匹配指定名称的进程发送 `SIGUSR1` 信号
- `pkill -o "firefox"` : 终止主 `firefox` 进程来关闭浏览器

## Links
- More information: https://manned.org/pkill
