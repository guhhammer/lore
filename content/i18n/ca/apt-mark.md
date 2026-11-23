# apt-mark

Eina per canviar l'estat dels paquets instal·lats.

## Usage
- `sudo apt-mark auto nom_paquet` : marca un paquet com a instal·lat automàticament
- `sudo apt-mark hold nom_paquet` : manté un paquet en la seva versió actual i evita que s'actualitzi
- `sudo apt-mark unhold nom_paquet` : permet que es pugui actualitzar de nou
- `apt-mark showmanual` : mostra els paquets instal·lats manualment
- `apt-mark showhold` : mostra els paquets mantinguts que no estàn actualitzats

## Links
- More information: https://manned.org/apt-mark
