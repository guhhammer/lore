# ss

Hulpprogramma om sockets te onderzoeken.

## Usage
- `ss -a --tcp|--udp|--raw|--unix` : toon alle TCP/UDP/RAW/UNIX-sockets
- `ss state|exclude bucket|big|connected|synchronized|...` : filter TCP-sockets op status, alleen/uitgezonderd
- `ss -t src :443` : toon alle TCP-sockets verbonden met de lokale HTTPS-poort (443)
- `ss -lt src :8080` : toon alle TCP-sockets die luisteren op de lokale poort 8080
- `ss -pt dst :ssh` : toon alle TCP-sockets samen met processen die verbonden zijn met een externe SSH-poort
- `ss -u 'sport == :bronpoort and dport == :bestemmingspoort'` : toon alle UDP-sockets verbonden op specifieke bron- en bestemmingspoorten
- `ss -4t src 192.168/16` : toon alle TCP IPv4-sockets lokaal verbonden op het subnet 192.168.0.0/16
- `ss -K dst ip_adres dport = poort` : beëindig een IPv4- of IPv6-socketverbinding met een specifiek bestemmings-IP en poort

## Links
- More information: https://manned.org/ss
