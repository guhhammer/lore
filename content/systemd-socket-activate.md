# systemd-socket-activate

Socket activation for systemd services.

## Usage
- `systemd-socket-activate path/to/socket.service` : activate a service when a specific socket is connected
- `systemd-socket-activate path/to/socket1.service path/to/socket2.service` : activate multiple sockets for a service
- `SYSTEMD_SOCKET_ACTIVATION=1 systemd-socket-activate path/to/socket.service` : pass environment variables to the service being activated
- `systemd-socket-activate path/to/socket.socket path/to/service.service` : activate a service along with a notification socket
- `systemd-socket-activate path/to/socket.service -l 8080` : activate a service with a specified port

## Categories
system, network, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-socket-activate
