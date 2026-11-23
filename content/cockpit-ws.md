# cockpit-ws

Communicate between the browser application and various configuration tools and services like `cockpit-bridge`.

## Usage
- `cockpit-ws --local-ssh` : start with authentication via SSH at `127.0.0.1` with port `22` enabled
- `cockpit-ws --port port` : start an HTTP server on a specific port
- `cockpit-ws --address ip_address` : start and bind to a specific IP address (defaults to `0.0.0.0`)
- `cockpit-ws --no-tls` : start without TLS
- `cockpit-ws --help` : display help

## Categories
network, system

## Links
- More information: https://cockpit-project.org/guide/latest/cockpit-ws.8.html
- tldr page: https://tldr.inbrowser.app/pages/linux/cockpit-ws
