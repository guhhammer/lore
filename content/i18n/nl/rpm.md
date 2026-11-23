# rpm

RPM Package Manager.

## Usage
- `rpm -q httpd` : toon versie van het httpd-pakket
- `rpm -qa 'mariadb*'` : toon versies van alle overeenkomende pakketten
- `rpm -U pad/naar/pakket.rpm --force` : installeer een pakket geforceerd, ongeacht de momenteel geïnstalleerde versies
- `rpm -qf /etc/postfix/main.cf` : identificeer de eigenaar van een bestand en toon de versie van het pakket
- `rpm -ql kernel` : toon bestanden die bij een pakket horen
- `rpm -qp --scripts pakket.rpm` : toon scriptlets uit een RPM-bestand
- `rpm -Va 'php-*'` : toon gewijzigde, missende en/of onjuist geïnstalleerde bestanden van overeenkomende pakketten
- `rpm -q --changelog pakket` : toon het changelog van een specifiek pakket

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
