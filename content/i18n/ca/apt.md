# apt

Eina de gestió de paquets per a distribucions basades en Debian. Es recomana substituïr-lo per `apt-get` quan es faci servir interactivament en Ubuntu 16.04 o en versions posteriors.

## Usage
- `sudo apt update` : actualitza la llista de paquets i versions disponbles (es recomana executar aquest comandament abans que qualsevol altre `apt`)
- `apt search paquet` : busca un paquet
- `apt show paquet` : mostra la informació de un paquet
- `sudo apt install paquet` : instala un paquet o l'actualitza a l'última versió disponible
- `sudo apt remove paquet` : elimina un paquet (si s'utiliza `purge` també elimina els seus arxius de configuració)
- `sudo apt upgrade` : actualitza tots els paquets instal·lats a les noves versions disponibles
- `apt list` : mostra tots els paquets
- `apt list -i` : mostra tots els paquets instalats

## Links
- More information: https://manned.org/apt.8
