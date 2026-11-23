# sudo

Voert een commando uit als de superuser of een andere gebruiker.

## Usage
- `sudo less /var/log/syslog` : voer een commando uit als de superuser
- `sudo -e /etc/fstab` : pas een bestand aan als superuser met jouw standaardeditor
- `sudo -u gebruiker -g groep id -a` : voer een commando uit als een andere gebruiker en/of groep
- `sudo !!` : herhaal het laatste commando met `sudo` ervoor (alleen in Bash, Zsh, etc.)
- `sudo -i` : start de standaard shell met superuserrechten en voer login-specifieke bestanden uit (`.profile`, `.bash_profile`, etc.)
- `sudo -s` : start de standaard shell met superuserrechten zonder de omgeving te veranderen
- `sudo -i -u gebruiker` : start de standaard shell als de opgegeven gebruiker, laad de omgeving van de gebruiker en lees login-specifieke bestanden (`.profile`, `.bash_profile`, etc.)
- `sudo -ll` : toon de toegestane (en verboden) commando's voor de aanroepende gebruiker in het lange formaat

## Links
- More information: https://www.sudo.ws/sudo.html
