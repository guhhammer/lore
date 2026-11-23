# zabbix_server

Core daemon of Zabbix software.

## Usage
- `zabbix_server` : start the server with the default configuration file
- `zabbix_server -c path/to/zabbix_server.conf` : start the server with a custom configuration file
- `zabbix_server -c path/to/zabbix_server.conf -f` : run the server in foreground
- `zabbix_server -c path/to/zabbix_server.conf -T` : test the configuration file and exit
- `zabbix_server -c path/to/zabbix_server.conf -R config_cache_reload` : reload configuration cache (runtime control)
- `zabbix_server -c path/to/zabbix_server.conf -R housekeeper_execute` : execute the housekeeper (runtime control)
- `zabbix_server -c path/to/zabbix_server.conf -R log_level_increase|decrease` : increase or decrease log level for all processes (runtime control)
- `zabbix_server -h` : display help

## Categories
files, system

## Links
- More information: https://manned.org/zabbix_server
- tldr page: https://tldr.inbrowser.app/pages/common/zabbix_server
