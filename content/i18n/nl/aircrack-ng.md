# aircrack-ng

Kraak WEP- en WPA/WPA2-sleutels van handshake in vastgelegde pakketten. Onderdeel van Aircrack-ng netwerksoftwaresuite.

## Usage
- `aircrack-ng -w pad/naar/woordenlijst.txt pad/naar/opnamebestand.cap` : kraak de sleutel uit het opnamebestand met behulp van de woordenlijst
- `aircrack-ng -p nummer -w pad/naar/woordenlijst.txt pad/naar/opnamebestand.cap` : kraak de sleutel met behulp van meerdere CPU-threads uit het opnamebestand met behulp van de woordenlijst
- `aircrack-ng -w pad/naar/woordenlijst.txt -e extended_ssid pad/naar/opnamebestand.cap` : kraak de sleutel uit het opnamebestand met behulp van de woordenlijst en de essid van het toegangspunt
- `aircrack-ng -w pad/naar/woordenlijst.txt --bssid mac pad/naar/opnamebestand.cap` : kraak de sleutel uit het opnamebestand met behulp van de woordenlijst en het MAC-adres van het toegangspunt

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=aircrack-ng
