# zabbix_server

Zabbix 软件的核心守护进程。

## Usage
- `zabbix_server` : 使用默认配置文件启动服务器
- `zabbix_server -c 路径/到/zabbix_server.conf` : 使用自定义配置文件启动服务器
- `zabbix_server -c 路径/到/zabbix_server.conf -f` : 在前台运行服务器
- `zabbix_server -c 路径/到/zabbix_server.conf -T` : 测试配置文件并退出
- `zabbix_server -c 路径/到/zabbix_server.conf -R config_cache_reload` : 重新加载配置缓存（运行时控制）
- `zabbix_server -c 路径/到/zabbix_server.conf -R housekeeper_execute` : 执行 housekeeper（运行时控制）
- `zabbix_server -c 路径/到/zabbix_server.conf -R log_level_increase|decrease` : 增加或减少所有进程的日志级别（运行时控制）
- `zabbix_server -h` : 显示帮助

## Links
- More information: https://manned.org/zabbix_server
