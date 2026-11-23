# airdecap-ng

Décrypte un fichier de capture WEP, WPA ou WPA2 chiffré. Fait partie de la suite logicielle réseau Aircrack-ng.

## Usage
- `airdecap-ng -b mac_pa chemin/vers/capture.cap` : supprime les en-têtes sans fil d'un fichier de capture réseau ouvert et utilise l'adresse MAC du point d'accès pour filtrer
- `airdecap-ng -w clé_hex chemin/vers.cap` : décrypte un fichier de capture wEP chiffré en utilisant la clé au format hexadécimal
- `airdecap-ng -e extended_ssid -p mot_de_passe chemin/vers.cap` : décrypte un fichier de capture WPA/WPA2 chiffré à l'aide de l'essid et du mot de passe du point d'accès
- `airdecap-ng -l -e extended_ssid -p mot_de_passe chemin/vers.cap` : décrypte un fichier de capture WPA/WPA2 chiffré en préservant les en-têtes à l'aide de l'essid et du mot de passe du point d'accès
- `airdecap-ng -b mac_pa -e extended_ssid -p mot_de_passe chemin/vers/capture.cap` : décrypte un fichier de capture WPA/WPA2 chiffré à l'aide de l'essid et du mot de passe du point d'accès et utilise son adresse MAC pour filtrer

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=airdecap-ng
