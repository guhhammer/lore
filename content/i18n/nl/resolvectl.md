# resolvectl

Resolve domeinnamen, IPv4 en IPv6 adressen, DNS resource records en services.

## Usage
- `resolvectl` : toon DNS instellingen
- `resolvectl query domein1 domein2 ...` : resolve de IPv4 en IPv6 adressen voor een of meerdere domeinen
- `resolvectl query ip_adres` : verkrijg het domein van een gespecificeerd IP adres
- `resolvectl flush-caches` : flush alle lokale DNS caches
- `resolvectl statistics` : toon DNS statistieken (transacties, cache en DNSSEC oordelen)
- `resolvectl --legend no -t MX query domein` : verkrijg een MX record van een domein
- `resolvectl service _service._protocol naam` : resolve een SRV record, bijvoorbeeld `_xmpp-server._tcp gmail.com`
- `resolvectl tlsa tcp domein:443` : verkrijg een TLS sleutel

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/resolvectl.html
