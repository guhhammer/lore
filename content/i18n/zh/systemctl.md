# systemctl

控制 systemd 系统和服务管理器。

## Usage
- `systemctl status` : 显示所有正在运行的服务
- `systemctl --failed` : 列出失败的单元
- `systemctl start|stop|restart|reload|status 单元` : 启动/停止/重启/重新加载/显示服务的状态
- `systemctl enable/disable 单元` : 启用/禁用开机时启动的单元
- `systemctl daemon-reload` : 重新加载 systemd，扫描新的或更改的单元
- `systemctl is-active|is-enabled|is-failed 单元` : 检查单元是否激活/启用/失败
- `systemctl list-units -t service|socket|automount --state failed|running` : 按运行/失败状态过滤列出所有服务/套接字/自动挂载单元
- `systemctl cat 单元` : 显示单元文件的内容和绝对路径

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
