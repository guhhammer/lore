# nc.openbsd

The OpenBSD netcat: reads and writes data over TCP or UDP connections, for testing ports and moving bytes between machines.

## Usage
- `nc.openbsd -zv host 80` : check whether a port is open
- `nc.openbsd -l 9000` : listen on a port and print what arrives
- `nc.openbsd host 9000 < file` : send a file to a listening machine
- `nc.openbsd -u host 53` : use UDP instead of TCP
- `nc.openbsd -w 5 host 80` : give up after 5 seconds

## Notes
Send only to machines you own or may test.

## Categories
network
