# airdecap-ng

Decodeer een WEP-, WPA- of WPA2-gecodeerd opnamebestand. Onderdeel van Aircrack-ng netwerksoftwaresuite.

## Usage
- `airdecap-ng -b ap_mac pad/naar/opnamebestand.cap` : verwijder draadloze headers uit een open netwerkopnamebestand en gebruik het MAC-adres van het toegangspunt om te filteren
- `airdecap-ng -w hex_key pad/naar/opnamebestand.cap` : decodeer een met wEP gecodeerd opnamebestand met de sleutel in hex-indeling
- `airdecap-ng -e extended_ssid -p wachtwoord pad/naar/opnamebestand.cap` : decodeer een met WPA/WPA2 gecodeerd opnamebestand met behulp van de essid en het wachtwoord (p) van het toegangspunt
- `airdecap-ng -l -e extended_ssid -p wachtwoord pad/naar/opnamebestand.cap` : decodeer een met WPA/WPA2 gecodeerd opnamebestand met behoud van de headers met behulp van de essid en het wachtwoord (p) van het toegangspunt
- `airdecap-ng -b ap_mac -e extended_ssid -p wachtwoord pad/naar/opnamebestand.cap` : decodeer een met WPA/WPA2 gecodeerd opnamebestand met behulp van de essid en het wachtwoord (p) van het toegangspunt en gebruik het MAC-adres om te filteren

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=airdecap-ng
