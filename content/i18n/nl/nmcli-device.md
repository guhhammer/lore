# nmcli device

Beheer netwerkinterfaces en zet nieuwe Wi-Fi-verbindingen op via NetworkManager.

## Usage
- `nmcli d` : toon de statussen van alle netwerkinterfaces
- `nmcli d w` : toon alle beschikbare Wi-Fi-toegangspunten
- `nmcli d w c ssid -a` : verbind met een Wi-Fi netwerk via een gespecificeerd SSID (je wordt gevraagd om een wachtwoord)
- `nmcli d w ho ifname wlan0 ssid "hotspot_ssid" password "wachtwoord"` : creëer een Wi-Fi-hotspot
- `nmcli d w s` : toon het wachtwoord en de QR-code voor het huidige Wi-Fi netwerk
- `nmcli d sh wlan0` : toon gedetailleerde informatie over een apparaat
