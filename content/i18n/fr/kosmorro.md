# kosmorro

Calcule les éphémérides et les évènements pour une date donnée, à un emplacement donné sur Terre.

## Usage
- `kosmorro -p "48.7996,2.3511"` : obtient les éphémérides pour Paris (France)
- `kosmorro -p "48.7996,2.3511" -t "Europe/Paris"` : obtient les éphémérides pour Paris (France), sur le fuseau horaire local
- `kosmorro -p "48.7996,2.3511" -d "2020-06-09"` : obtient les éphémérides du 9 juin 2020 pour Paris (France)
- `kosmorro -p "48.7996,2.3511" -d "+2d"` : obtient les éphémérides pour Paris (France) dans deux jours
- `kosmorro -o "chemin/vers/fichier.pdf"` : génère un fichier PDF (TeXLive doit être installé)

## Links
- More information: https://kosmorro.space/cli/manpage/
