# nmcli general

Spravuje obecné nastavení NetworkManageru.

## Usage
- `nmcli g` : zobrazit obecný stav NetworkManageru
- `nmcli g h` : zobrazit hostitelské jméno aktuálního zařízení
- `sudo nmcli g h nove_hostitelske_jmeno` : změnit hostitelské jméno aktuálního zařízení
- `nmcli g p` : zobrazit oprávění NetworkManageru
- `nmcli g l` : zobrazit aktuální úroveň logů a domén
- `sudo nmcli g l l INFO|OFF|ERR|WARN|DEBUG|TRACE domain domena_1,domena_2,...` : nastavit úroveň logů a/nebo domén (všechny dostupné domény najdete pomocí `man NetworkManager.conf`)
