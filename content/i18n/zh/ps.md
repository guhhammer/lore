# ps

提供正在运行的进程的信息。

## Usage
- `ps aux` : 列出所有正在运行的进程
- `ps auxww` : 列出所有正在运行的进程，包括完整的命令字符串
- `ps aux | grep 字符串` : 查找与字符串匹配的进程
- `ps -u $(id -u) -F` : 以 extra full 格式列出当前用户的所有进程
- `ps -u $(id -u) f` : 以树形方式列出当前用户的所有进程
- `ps -o ppid= -p 进程id` : 获取一个进程的父进程 ID
- `ps --sort size` : 按内存使用量对进程进行排序

## Links
- More information: https://manned.org/ps
