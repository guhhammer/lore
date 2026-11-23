# netserver

Server-side command for `netperf`, the benchmarking application that measures network throughput.

## Usage
- `netserver` : start a server on the default port (12865) and fork to background
- `netserver -D` : start server in foreground and do not fork
- `netserver -p port` : specify port
- `netserver -4|6` : force IPv[4] or IPv[6]

## See also
- netperf

## Categories
network

## Links
- More information: https://manned.org/netserver
- tldr page: https://tldr.inbrowser.app/pages/common/netserver
