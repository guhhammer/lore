# git tag

Creëer, toon, verwijder of verifieer tags. Een tag is een statische verwijzing naar een commit.

## Usage
- `git tag` : toon alle tags
- `git tag tag_naam` : maak een tag aan met de opgegeven naam die verwijst naar de huidige commit
- `git tag tag_naam commit` : maak een tag aan met de opgegeven naam die verwijst naar een gegeven commit
- `git tag tag_naam -m tag_bericht` : maak een geannoteerde tag aan met het opgegeven bericht
- `git tag -d tag_naam` : verwijder de tag met de opgegeven naam
- `git fetch -t` : haal geüpdatete tags op van de externe repository
- `git push --tags` : push alle tags naar een externe repository
- `git tag --contains commit` : toon alle tags die een bepaalde commit bevatten (`HEAD` indien niet gespecificeerd)

## Links
- More information: https://git-scm.com/docs/git-tag
