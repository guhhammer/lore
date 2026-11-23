# htop

显示正在运行的进程的动态实时信息。`top` 的增强版。

## Usage
- `htop` : 启动 `htop`
- `htop -u 用户名` : 启动 `htop`, 显示指定用户拥有的进程
- `htop -t` : 以树形视图显示进程的层级关系，展示父子进程之间的关系
- `htop -s sort_item` : 使用指定的 `sort_item` 对进程排序（使用 `htop --sort help` 获取可用选项）
- `htop -d 50` : 以指定的更新间隔启动 `htop`, 以十分之一秒为单位（即 50 = 5 秒）
- `<?>` : 运行 `htop` 时查看交互式命令
- `<Tab>` : 切换到不同的标签
- `htop -h` : 显示帮助

## Links
- More information: https://manned.org/htop
