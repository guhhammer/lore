# ping

Verstuur ICMP ECHO_REQUEST-pakketten naar netwerkhosts.

## Usage
- `ping 10.0.0.1` : ping een host met IP-adres `10.0.0.1` (Opmerking: `10.0.0.1` kan worden afgekort tot `10.1`)
- `ping -c aantal host` : ping een host een specifiek aantal keren
- `ping -i seconden host` : ping host met een specifiek interval in seconden tussen verzoeken (standaard is 1 seconde)
- `ping -n host` : ping host zonder te proberen symbolische namen voor adressen op te zoeken
- `ping -a host` : ping host en laat een bel afgaan wanneer een pakket wordt ontvangen (als je terminal dit ondersteunt)
- `ping -O host` : toon ook een bericht als er geen reactie is ontvangen
- `ping -c aantal -W seconden -w seconden host` : ping een host met een specifiek aantal (c) pings, timeout om te Wachten voor elk antwoord, en totale wachttijd voor de gehele ping-uitvoering
- `ping -6 ff02::1%eth0` : ping alle IPv6-hosts in het lokale netwerk

## Links
- More information: https://manned.org/ping
