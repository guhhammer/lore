# gdbus

Interact with D-Bus objects. Part of GLib.

## Usage
- `gdbus introspect --session --dest destination_bus_name --object-path /path/to/object` : introspect an object to see its interfaces and methods
- `gdbus call --session --dest destination_bus_name --object-path /path/to/object --method interface.method_name argument1 argument2 ...` : call a method on an object with arguments
- `gdbus emit --session --object-path /path/to/object --signal interface.signal_name argument1 argument2 ...` : emit a signal from an object with arguments
- `gdbus monitor --session` : monitor all messages on the session bus

## Categories
files, network

## Links
- More information: https://manned.org/gdbus
- tldr page: https://tldr.inbrowser.app/pages/linux/gdbus
