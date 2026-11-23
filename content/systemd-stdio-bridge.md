# systemd-stdio-bridge

Implement a proxy between `stdin`/`stdout` and a D-Bus. Note: It expects to receive an open connection via `stdin`/`stdout` when started, and will create a new connection to the specified bus.

## Usage
- `systemd-stdio-bridge` : forward `stdin`/`stdout` to the local system bus
- `systemd-stdio-bridge --user` : forward `stdin`/`stdout` to a specific user's D-Bus
- `systemd-stdio-bridge -M mycontainer` : forward `stdin`/`stdout` to the local system bus within a specific container
- `systemd-stdio-bridge -p unix:path=/path/to/dbus_socket` : forward `stdin`/`stdout` to a custom D-Bus address

## Categories
system, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-stdio-bridge
