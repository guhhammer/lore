# apt-file

Busca arxius en paquets APT, incloent els que encara no s'han instal·lat.

## Usage
- `sudo apt update` : actualita les metadades de la base de dades
- `apt-file find ruta/al/arxiu` : busca paquets que continguin l'arxiu o ruta especificada
- `apt-file list nom_paquet` : mostra el contingut del paquet especificat
- `apt-file find -x expressió_regular` : busca paquets que igualin l'expressió regular donada en `patró`

## Links
- More information: https://manned.org/apt-file
