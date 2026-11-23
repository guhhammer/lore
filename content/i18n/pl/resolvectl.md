# resolvectl

Znajdź nazwy domen, adresy IPv4 i IPv6, rekordy zasobów DNS i usługi. Analizuj i rekonfiguruj resolwer DNS.

## Usage
- `resolvectl status` : wyświetl ustawienia DNS
- `resolvectl query domena1 domena2 ...` : znajdź adresy IPv4 i IPv6 jednej lub więcej domen
- `resolvectl query adres_ip` : znajdź domenę podanego adresu IP
- `resolvectl --legend=no --type=MX query domena` : znajdź rekord MX podanej domeny
- `resolvectl service _usługa._protokół nazwa` : znajdź rekord SRV, na przykład _xmpp-server._tcp gmail.com
- `resolvectl openpgp email` : pobierz klucz publiczny z adresu email z rekordu DNS OPENPGPGKEY
- `resolvectl tlsa tcp domena:443` : znajdź klucz TLS

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/resolvectl.html
