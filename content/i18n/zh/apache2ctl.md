# apache2ctl

Apache HTTP web 服务器命令行管理工具。 基于 Debian 的操作系统自带该命令，基于 RHEL 的查看 `httpd`。

## Usage
- `sudo apache2ctl start` : 启动 Apache 守护进程。如果已运行则发送一个消息
- `sudo apache2ctl stop` : 关闭 Apache 守护进程
- `sudo apache2ctl restart` : 重启 Apache 守护进程
- `sudo apache2ctl -t` : 检查配置文件语法
- `sudo apache2ctl -M` : 列出已加载模块

## Links
- More information: https://manned.org/apache2ctl
