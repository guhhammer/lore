# networkctl

Zapytaj o stan łączy sieciowych. Zarządzaj konfiguracją sieci za pomocą `systemd-networkd`.

## Usage
- `networkctl` : wyświetl listę istniejących łączy i ich status
- `networkctl status` : wyświetl ogólny status sieci
- `sudo networkctl up interfejs1 interfejs2 ...` : włącz urządzenia sieciowe
- `sudo networkctl down interfejs1 interfejs2 ...` : wyłącz urządzenia sieciowe
- `sudo networkctl renew interfejs1 interfejs2 ...` : odnów konfiguracje dynamiczne (np. adresy IP przydzielone przez serwer DHCP)
- `sudo networkctl reload` : przeładuj pliki konfiguracyjne (`.netdev` i `.network`)
- `sudo networkctl reconfigure interfejs1 interfejs2 ...` : rekonfiguruj interfejsy sieciowe (jeżeli pliki konfiguracyjne były edytowane, najpierw uruchom `networkctl reload`)

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/networkctl.html
