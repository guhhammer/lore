# pacman-mirrors

Genereer een `pacman`-mirrorlist voor Manjaro Linux. Elke uitvoering van `pacman-mirrors` vereist dat je je database synchroniseert en je systeem bijwerkt met `sudo pacman -Syyu`.

## Usage
- `sudo pacman-mirrors --fasttrack` : genereer een mirrorlist met de standaardinstellingen
- `pacman-mirrors --status` : verkrijg de status van de huidige mirrors
- `pacman-mirrors --get-branch` : toon de huidige branch
- `sudo pacman-mirrors --api --set-branch stable|unstable|testing` : schakel over naar een andere branch
- `pacman-mirrors --country-list` : toon de beschikbare landen
- `sudo pacman-mirrors --country United_States,France,Germany,...` : genereer een mirrorlist met alleen mirrors in specifieke landen

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors
