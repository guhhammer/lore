# mv

Flytt eller gi nytt navn til filer og mapper.

## Usage
- `mv sti/til/kilde sti/til/mål` : gi nytt navn til en fil eller mappe når målet ikke er en eksisterende mappe
- `mv sti/til/kilde sti/til/eksisterende_mappe` : flytt en fil eller mappe inn i en eksisterende mappe
- `mv sti/til/kilde1 sti/til/kilde2 ... sti/til/eksisterende_mappe` : flytt flere filer inn i en eksisterende mappe, og behold filnavnene uendret
- `mv -f sti/til/kilde sti/til/mål` : ikke spør om bekreftelse før eksisterende filer overskrives
- `mv -i sti/til/kilde sti/til/mål` : spør om bekreftelse interaktivt før eksisterende filer overskrives, uavhengig av filrettigheter
- `mv -n sti/til/kilde sti/til/mål` : ikke overskriv eksisterende filer på målet
- `mv -v sti/til/kilde sti/til/mål` : flytt filer i detaljert modus, og vis filer etter at de er flyttet
- `find /var/log -type f -name '*.log' -print0 | xargs -0 mv -t sti/til/målmappe` : angi målmappe slik at du kan bruke eksterne verktøy for å samle filer som skal flyttes

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
