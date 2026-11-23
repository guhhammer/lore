# stow

Symlink-beheerder. Vaak gebruikt om dotfiles te beheren.

## Usage
- `stow -t pad/naar/doel_map pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : symlink alle bestanden recursief naar de opgegeven map
- `stow -D -t pad/naar/doel_map pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : verwijder alle symlinks recursief in de opgegeven map
- `stow -n -t pad/naar/doel_map pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : simuleer om te zien hoe het resultaat eruit ziet
- `stow -R -t pad/naar/doel_map pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : verwijder en maak opnieuw de symlinks aan
- `stow --ignore=regex -t pad/naar/doel_map pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : sluit bestanden uit die overeenkomen met een `regex`

## Links
- More information: https://www.gnu.org/software/stow/manual/stow.html#Invoking-Stow
