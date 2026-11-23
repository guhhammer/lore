# apt-get

Eina de gestió de paquets per a distribucions basades en Debian. Busca paquets utilizant `apt-cache`.

## Usage
- `sudo apt-get update` : actualitza la llista de paquets i versions disponibles (es recomana executar aquest comandament abans que qualsevol altre `apt-get`)
- `sudo apt-get install paquet` : instala un paquet o l'actualitza a l'última versió disponible
- `sudo apt-get remove paquet` : elimina un paquet
- `sudo apt-get purge paquet` : elimina un paquet i els seus arxius de configuració
- `sudo apt-get upgrade` : actualitza tots els paquets instal·lats a les noves versions disponibles
- `sudo apt-get autoclean` : neteja el repositori local - eliminant fitxers de paquet (`.deb`) de descàrregues interrompudes que ja no es poden descarregar
- `sudo apt-get autoremove` : elimina tots els paquets inneccessaris
- `sudo apt-get dist-upgrade` : actualitza paquets instal·lats (com `upgrade`), però elimina els paquets obsolets i instal·la paquets adicionals per satisfer les dependències

## Links
- More information: https://manned.org/apt-get.8
