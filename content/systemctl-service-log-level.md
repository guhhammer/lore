# systemctl service-log-level

Get or set the runtime log level of a service via D-Bus.

## Usage
- `systemctl service-log-level service_name` : show the current log level of a service
- `systemctl service-log-level service_name emerg|alert|crit|err|warning|notice|info|debug` : set the log level of a service (the level name can be replaced with a number from 0 to 7)

## Categories
monitoring, system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-service-log-level
