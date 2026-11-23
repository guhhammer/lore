# semanage port

Beheer persistente SELinux-poortdefinities.

## Usage
- `sudo semanage port -l` : toon alle poortlabelregels
- `sudo semanage port -l -C -n` : toon alle door de gebruiker gedefinieerde poortlabelregels zonder koppen
- `sudo semanage port -a -t ssh_port_t -p tcp 22000` : voeg een door de gebruiker gedefinieerde regel toe die een label toekent aan een protocol-poortpaar
- `sudo semanage port -a -t http_port_t -p tcp 80-88` : voeg een door de gebruiker gedefinieerde regel toe die een label toekent aan een protocol-poortbereikpaar
- `sudo semanage port -d -p udp 11940` : verwijder een door de gebruiker gedefinieerde regel met behulp van zijn protocol-poortpaar

## Links
- More information: https://manned.org/semanage-port
