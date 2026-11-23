# ping6

Sendet ICMP ECHO_REQUEST Pakete an andere Geräte im Netzwerk, welche mittels IPv6-Adressen identifiziert werden.

## Usage
- `ping6 ziel` : sende Pings an ein Gerät im Netzwerk
- `ping6 -c anzahl ziel` : sende nur eine bestimmte Anzahl an Pings
- `ping6 -i sekunden ziel` : sende Pings und bestimme das Interval in Sekunden zwischen diesen (standardmäßig ist es eine Sekunde)
- `ping6 -n ziel` : sende Pings ohne symbolische Namen nach Adressen aufzulösen
- `ping6 -a ziel` : sende Pings und signalisiere eine erfolgreiche Antwort durch ein Bell Signal (wenn das Terminal es unterstützt)

## Links
- More information: https://manned.org/ping6
