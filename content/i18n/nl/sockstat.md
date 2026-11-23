# sockstat

Toon open Internet- of UNIX-domeinsockets.

## Usage
- `sockstat` : toon informatie voor IPv4- en IPv6-sockets voor zowel luister- als verbonden sockets
- `sockstat -4|6 -l -R tcp|udp|raw|unix -p poort1,poort2...` : toon informatie voor IPv[4]/IPv[6] sockets die luisteren op specifieke poorten met een specifiek pRotocol
- `sockstat -cu` : toon ook verbonden (c) sockets en unix-sockets
- `sockstat -P pid|proces` : toon alleen sockets van het opgegeven `pid` of proces
- `sockstat -U uid|gebruiker` : toon alleen sockets van de opgegeven `uid` of gebruiker
- `sockstat -G gid|groep` : toon alleen sockets van de opgegeven `gid` of groep

## Links
- More information: https://manned.org/sockstat
