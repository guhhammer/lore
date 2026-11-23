# busctl

Introspect and monitor the D-Bus bus.

## Usage
- `busctl` : show all peers on the bus by their service names
- `busctl status service|pid` : show process information and credentials of a bus service, a process, or the owner of the bus (if no parameter is specified)
- `busctl monitor service1 service2 ...` : dump messages being exchanged. If no service is specified, show all messages on the bus
- `busctl tree service1 service2 ...` : show an object tree of one or more services (or all services if no service is specified)
- `busctl introspect service path/to/object` : show interfaces, methods, properties, and signals of the specified object on the specified service
- `busctl get-property service path/to/object interface_name property_name` : retrieve the current value of one or more object properties
- `busctl call service path/to/object interface_name method_name` : invoke a method and show the response

## Categories
system, monitoring, files

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/busctl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/busctl
