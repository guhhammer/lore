# zabbix_agent2

用于监控服务器参数的守护进程。

## Usage
- `zabbix_agent2` : 使用默认配置文件启动代理
- `zabbix_agent2 -c 路径/到/zabbix_agent2.conf` : 使用自定义配置文件启动代理
- `zabbix_agent2 -c 路径/到/zabbix_agent2.conf -T` : 测试配置文件并退出
- `zabbix_agent2 -c 路径/到/zabbix_agent2.conf -t 监控项键值 -v` : 测试指定监控项并输出详细信息
- `zabbix_agent2 -c 路径/到/zabbix_agent2.conf -R userparameter_reload` : 从配置文件中重新加载用户参数（运行时控制）
- `zabbix_agent2 -c 路径/到/zabbix_agent2.conf -R loglevel increase|decrease` : 增加或减少日志级别（运行时控制）
- `zabbix_agent2 -h` : 显示帮助

## Links
- More information: https://manned.org/zabbix_agent2
