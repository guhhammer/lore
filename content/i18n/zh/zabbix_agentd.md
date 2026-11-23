# zabbix_agentd

用于监控服务器参数的守护进程。

## Usage
- `zabbix_agentd` : 使用默认配置文件启动 agent
- `zabbix_agentd -c 路径/到/zabbix_agentd.conf` : 使用自定义配置文件启动 agent
- `zabbix_agentd -c 路径/到/zabbix_agentd.conf -f` : 在前台运行 agent（保持附着在当前终端会话中）
- `zabbix_agentd -c 路径/到/zabbix_agentd.conf -T` : 测试配置文件
- `zabbix_agentd -c 路径/到/zabbix_agentd.conf -t 监控项键值 -v` : 使用详细输出测试指定的监控项
- `zabbix_agentd -c 路径/到/zabbix_agentd.conf -R userparameter_reload` : 从配置文件重新加载用户参数（运行时控制）
- `zabbix_agentd -c 路径/到/zabbix_agentd.conf -R log_level_increase|decrease` : 提高或降低所有进程的日志级别（运行时控制）
- `zabbix_agentd -h` : 显示帮助

## Links
- More information: https://manned.org/zabbix_agentd
