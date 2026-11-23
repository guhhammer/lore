# zabbix_agentd

Daemon for monitoring server parameters.

## Usage
- `zabbix_agentd` : start the agent with the default configuration file
- `zabbix_agentd -c path/to/zabbix_agentd.conf` : start the agent with a custom configuration file
- `zabbix_agentd -c path/to/zabbix_agentd.conf -f` : run the agent in foreground (it stays attached to the current terminal session)
- `zabbix_agentd -c path/to/zabbix_agentd.conf -T` : test the configuration file
- `zabbix_agentd -c path/to/zabbix_agentd.conf -t item_key -v` : test a specific item with verbose output
- `zabbix_agentd -c path/to/zabbix_agentd.conf -R userparameter_reload` : reload user parameters from the configuration file (runtime control)
- `zabbix_agentd -c path/to/zabbix_agentd.conf -R log_level_increase|decrease` : increase or decrease log level for all processes (runtime control)
- `zabbix_agentd -h` : display help

## Categories
files, system

## Links
- More information: https://manned.org/zabbix_agentd
- tldr page: https://tldr.inbrowser.app/pages/common/zabbix_agentd
