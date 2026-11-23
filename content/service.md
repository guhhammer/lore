# service

Manage services by running init scripts. The full script path should be omitted (`/etc/init.d/` is assumed).

## Usage
- `service --status-all` : list the name and status of all services
- `service service_name start|stop|restart|reload` : start/Stop/Restart/Reload service (start/stop should always be available)
- `service service_name --full-restart` : do a full restart (runs script twice with start and stop)
- `service service_name status` : show the current status of a service

## Categories
system, development, files

## Links
- More information: https://manned.org/service
- tldr page: https://tldr.inbrowser.app/pages/linux/service
