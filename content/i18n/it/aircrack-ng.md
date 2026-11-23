# aircrack-ng

Crackare chiavi WEP e WPA/WPA2 dall'handshake in pacchetti catturati. Parte della suite di software di rete Aircrack-ng.

## Usage
- `aircrack-ng -w percorso/alla/wordlist.txt percorso/al/capture.cap` : crackare la chiave dal file di cattura utilizzando wordlist
- `aircrack-ng -p number -w percorso/alla/wordlist.txt percorso/al/capture.cap` : crackare la chiave utilizzando più thread CPU dal file di cattura usando wordlist
- `aircrack-ng -w percorso/alla/wordlist.txt -e extended_ssid percorso/al/capture.cap` : crackare la chiave dal file di cattura utilizzando wordlist e l'essid del punto di accesso
- `aircrack-ng -w percorso/alla/wordlist.txt --bssid mac percorso/al/capture.cap` : crackare la chiave dal file di cattura utilizzando wordlist e l'indirizzo MAC del punto di accesso

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=aircrack-ng
