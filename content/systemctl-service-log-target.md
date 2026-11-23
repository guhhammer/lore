# systemctl service-log-target

Get or set the log target for a service. Only works for D-Bus integrated services.

## Usage
- `systemctl service-log-target service_name` : show the current log target for a service
- `systemctl service-log-target service_name console` : set the log target to `console` (send logs to `stderr`)
- `systemctl service-log-target service_name journal` : set the log target to `journal` (send logs to `systemd-journald`)
- `systemctl service-log-target service_name syslog` : set the log target to `syslog` (send logs to `/dev/log`)
- `systemctl service-log-target service_name auto` : allow systemd to choose an appropriate log target
- `systemctl service-log-target service_name null` : disable all log output

## Categories
system, monitoring

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-service-log-target
