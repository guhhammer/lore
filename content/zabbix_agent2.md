# zabbix_agent2

Daemon for monitoring server parameters.

## Usage
- `zabbix_agent2` : start the agent with the default configuration file
- `zabbix_agent2 -c path/to/zabbix_agent2.conf` : start the agent with a custom configuration file
- `zabbix_agent2 -c path/to/zabbix_agent2.conf -T` : test the configuration file and exit
- `zabbix_agent2 -c path/to/zabbix_agent2.conf -t item_key -v` : test a specific item with verbose output
- `zabbix_agent2 -c path/to/zabbix_agent2.conf -R userparameter_reload` : reload user parameters from the configuration file (runtime control)
- `zabbix_agent2 -c path/to/zabbix_agent2.conf -R loglevel increase|decrease` : increase or decrease log level (runtime control)
- `zabbix_agent2 -h` : display help

## Categories
files, system

## Links
- More information: https://manned.org/zabbix_agent2
- tldr page: https://tldr.inbrowser.app/pages/common/zabbix_agent2
