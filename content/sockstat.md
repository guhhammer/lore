# sockstat

List open Internet or UNIX domain sockets.

## Usage
- `sockstat` : show information for IPv4 and IPv6 sockets for both listening and connected sockets
- `sockstat -4|6 -l -R tcp|udp|raw|unix -p port1,port2...` : show information for IPv[4]/IPv[6] sockets listening on specific ports using a specific pRotocol
- `sockstat -cu` : also show connected sockets and unix sockets
- `sockstat -P process_id|process_name` : only show sockets of the specified `process_id` or process
- `sockstat -U user_id|username` : only show sockets of the specified `user_id` or user
- `sockstat -G group_id|group_name` : only show sockets of the specified `group_id` or group

## See also
- netstat

## Links
- More information: https://manned.org/sockstat
- tldr page: https://tldr.inbrowser.app/pages/linux/sockstat
