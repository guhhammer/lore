# apt-key

Eina de gestió de claus per al Gestor de Paquets APT (APT Package Manager) en Debian i Ubuntu.

## Usage
- `apt-key list` : mostra les claus de confiança
- `apt-key add arxiu_clau_pública.asc` : afegeix una clau al magatzem de claus de confiança
- `apt-key del id_clau` : borra una clau del magatzem de claus de confiança
- `wget -qO - https://host.tld/archiu.clau | apt-key add -` : afegir una clau remota al magatzem de claus de confiança
- `apt-key adv --keyserver pgp.mit.edu --recv id_clau` : afegir una clau d'un servidor de claus amb l'identificador de la clau

## Links
- More information: https://manned.org/apt-key
