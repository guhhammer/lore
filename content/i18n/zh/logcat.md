# logcat

转储系统消息日志，包括发生错误时的堆栈跟踪信息，以及应用程序记录的信息消息。

## Usage
- `logcat` : 显示系统日志
- `logcat -f 路径/到/文件` : 将系统日志写入文件（file）
- `logcat --regex 正则表达式` : 显示与`regex`式匹配的日志行
- `logcat --pid pid` : 显示特定 PID 的日志
- `logcat --pid $(pidof -s 包)` : 显示特定包的进程日志

## Links
- More information: https://developer.android.com/tools/logcat
