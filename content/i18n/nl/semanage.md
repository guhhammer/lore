# semanage

Hulpprogramma voor permanent SELinux-beleidsbeheer. Sommige subcommando's zoals `boolean`, `fcontext`, `port`, etc. hebben hun eigen documentatie.

## Usage
- `sudo semanage boolean -m --on|--off haproxy_connect_any` : stel een SELinux-boolean in of uit. Booleans stellen de beheerder in staat om aan te passen hoe beleidsregels invloed hebben op ingesloten procestypes (ook wel domeinen genoemd)
- `sudo semanage fcontext -a -t samba_share_t '/mnt/share(/.*)?'` : voeg een door de gebruiker gedefinieerde bestandscontextlabelregel toe. Bestandscontexten definiëren welke bestanden ingesloten domeinen mogen openen
- `sudo semanage port -a -t ssh_port_t -p tcp 22000` : voeg een door de gebruiker gedefinieerde poortlabelregel toe. Poortlabels definiëren op welke poorten ingesloten domeinen mogen luisteren
- `sudo semanage permissive --add|--delete httpd_t` : stel de permissieve modus in of uit voor een ingesloten domein. Per-domein permissieve modus biedt meer gedetailleerde controle vergeleken met `setenforce`
- `sudo semanage export -f pad/naar/bestand` : exporteer lokale aanpassingen in de standaardopslag
- `sudo semanage import -f pad/naar/bestand` : importeer een bestand gegenereerd door `semanage export` in lokale aanpassingen (VOORZICHTIG: kan huidige aanpassingen verwijderen!)

## Links
- More information: https://manned.org/semanage
