# socat1

A relay that connects two data channels, such as a file, a socket, a port or a terminal, and copies bytes between them.

## Usage
- `socat - TCP:host:80` : talk to a TCP port from your terminal
- `socat TCP-LISTEN:8080,fork TCP:host:80` : forward a local port to another host
- `socat -u FILE:in.txt TCP:host:9000` : send a file over TCP
- `socat UNIX-LISTEN:/tmp/sock,fork TCP:host:80` : expose a TCP service through a unix socket

## Notes
A forward like this opens a door in the network. Close it when you are done.

## Categories
network, files, shell
