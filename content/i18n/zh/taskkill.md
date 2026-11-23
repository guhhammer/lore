# taskkill

按进程 ID 或进程名终止进程。

## Usage
- `taskkill /pid 进程 id` : 通过进程 ID 终止进程
- `taskkill /im 进程名` : 通过进程名终止进程
- `taskkill /pid 进程名 /f` : 强制终止一个指定的进程
- `taskkill /im 进程名 /t` : 终止一个进程及其子进程
- `taskkill /pid 进程 id /s 远程主机名` : 终止远程计算机上的进程
- `taskkill /?` : 显示帮助
