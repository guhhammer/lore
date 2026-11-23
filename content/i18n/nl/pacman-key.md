# pacman-key

Wrapper-script voor GnuPG, gebruikt om de keyring van pacman te beheren.

## Usage
- `sudo pacman-key --init` : initialiseer de keyring van `pacman`
- `sudo pacman-key --populate` : voeg de standaard Arch Linux-sleutels toe
- `pacman-key -l` : toon sleutels uit de publieke keyring
- `sudo pacman-key -a pad/naar/sleutelbestand.gpg` : voeg de gespecificeerde sleutels toe
- `sudo pacman-key -r "uid|naam|email"` : ontvang een sleutel van een keyserver
- `pacman-key -f "uid|naam|email"` : toon de vingerafdruk van een specifieke sleutel
- `sudo pacman-key --lsign-key "uid|naam|email"` : onderteken een geïmporteerde sleutel lokaal
- `sudo pacman-key -d "uid|naam|email"` : verwijder een specifieke sleutel

## Links
- More information: https://manned.org/pacman-key
