# tftp

Trivial File Transfer Protocol-client.

## Usage
- `tftp server_ip poort` : maak verbinding met een TFTP-server met een interactieve shell, waarbij het IP-adres en de poort worden opgegeven
- `tftp server_ip -c commando` : maak verbinding met een TFTP-server en voer een TFTP-commando uit
- `tftp server_ip -6 -R poort:poort` : maak verbinding met een TFTP-server met IPv6 en forceer dat de oorspronkelijke poort binnen een Range ligt
- `mode binary|ascii` : [Interactief] Stel de overdrachtsmodus in op binaire of ASCII via de tftp-client
- `get bestand` : [Interactief] Download een bestand van een server via de tftp-client
- `put bestand` : [Interactief] Upload een bestand naar een server via de tftp-client
- `quit` : [Interactief] Verlaat de tftp-client

## Links
- More information: https://manned.org/tftp
