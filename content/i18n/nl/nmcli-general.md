# nmcli general

Beheer algemene instellingen van NetworkManager.

## Usage
- `nmcli g` : toon de algemene status van NetworkManager
- `nmcli g h` : toon de hostname van het huidige apparaat
- `sudo nmcli g h nieuwe_hostnaam` : verander de hostname van het huidige apparaat
- `nmcli g p` : toon de permissies van NetworkManager
- `nmcli g l` : toon het huidige logging level en domeinen
- `sudo nmcli g l l INFO|OFF|ERR|WARN|DEBUG|TRACE domain domein_1,domein_2,...` : zet het logging level en/of domeinen (zie `man NetworkManager.conf` voor alle beschikbare domeinen)
