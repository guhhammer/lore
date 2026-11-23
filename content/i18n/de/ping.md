# ping

Sendet ICMP ECHO_REQUEST Pakete an andere Geräte im Netzwerk.

## Usage
- `ping ziel` : sende Pings an ein Gerät im Netzwerk
- `ping -c anzahl ziel` : sende nur eine bestimmte Anzahl an Pings
- `ping -i sekunden ziel` : sende Pings und bestimme das Interval in Sekunden zwischen diesen (standardmäßig ist es eine Sekunde)
- `ping -n ziel` : sende Pings ohne symbolische Namen nach Adressen aufzulösen
- `ping -a ziel` : sende Pings und signalisiere eine erfolgreiche Antwort durch ein Bell Signal (wenn das Terminal es unterstützt)
- `ping -O ziel` : zeige auch eine Nachricht, wenn keine Antwort empfangen wurde

## Links
- More information: https://manned.org/ping
