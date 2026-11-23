# nmcli device

Spravuje síťové rozhraní a navazuje nové Wi-Fi spojení pomocí NetworkManageru.

## Usage
- `nmcli d` : vypsat stav všech síťových rozhraní
- `nmcli d w` : vypsat všechny dostupné přístupové body Wi-Fi
- `nmcli d w c ssid -a` : připojit se k Wi-Fi síťi s uvedeným SSID (budete vyzváni k zadání hesla)
- `nmcli d w ho ifname wlan0 ssid "hotspot_ssid" password "heslo"` : vytvořit Wi-Fi hotspot
- `nmcli d w s` : vypsat heslo a QR kód pro aktuální Wi-Fi síť
- `nmcli d sh wlan0` : vypsat podrobné informace o zařízení
