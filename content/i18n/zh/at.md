# at

在稍后时间执行命令。 结果将被发送到用户的邮箱。

## Usage
- `at now + 5 minutes` : 交互式创建命令并在 5 分钟后执行（完成后按 `<Ctrl d>`）
- `at 小时:分钟` : 交互式创建命令并在特定时间执行
- `echo "命令" | at 1000` : 在今天 10:00 AM 从标准输入执行一个命令
- `at -f 路径/到/文件 9:30 PM Tue` : 在下周二晚上 9:30 从指定文件执行命令
- `at -l` : 列出当前用户的所有待执行任务（等同于 `atq`）
- `at -c 任务编号` : 查看指定任务

## Links
- More information: https://manned.org/at
