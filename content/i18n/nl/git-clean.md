# git clean

Verwijder bestanden die niet worden bijgehouden door Git uit de werkmap.

## Usage
- `git clean -i` : verwijder niet-bijgehouden bestanden interactief
- `git clean -n` : toon welke bestanden verwijderd zouden worden zonder ze echt te verwijderen
- `git clean -f` : verwijder onmiddellijk alle niet-bijgehouden bestanden geforceerd
- `git clean -f -d` : verwijder niet-bijgehouden mappen (d)
- `git clean -f -- pad/naar/map '*.ext'` : verwijder alleen niet-bijgehouden bestanden die overeenkomen met specifieke paden of `glob`-patronen
- `git clean -f -e '*.ext' -e pad/naar/map/` : verwijder niet-bijgehouden bestanden, behalve die welke overeenkomen met de opgegeven patronen
- `git clean -f -x` : verwijder niet-bijgehouden bestanden, inclusief genegeerde (x) bestanden (bestanden die worden genegeerd door `.gitignore` en `.git/info/exclude`)

## Links
- More information: https://git-scm.com/docs/git-clean
