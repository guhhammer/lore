# grep

Find mønstre i filer via regulære udtryk (`regex`).

## Usage
- `grep "søgemønster" sti/til/fil` : søg efter et mønster i en fil
- `grep -F "eksakt_streng" sti/til/fil` : søg efter en eksakt streng (deaktiverer regulære udtryk)
- `grep -rnI "søgemønster" sti/til/mappe` : søg efter et mønster i alle filer, pånær binære, rekursivt i en mappe. Vis linjenumre der matcher til mønstret
- `grep -Ei "søgemønster" sti/til/fil` : brug udvidede regulære udtryk (understøtter `?`, `+`, `{}`, `()`, og `|`), i case-insensitive modus
- `grep --context|--before-context|--after-context 3 "søgemønster" sti/til/fil` : print 3 linjer af kontekst omkring, før eller efter hvert match
- `grep -Hn --color=always "søgemønster" sti/til/fil` : print, filnavn og linjenummer for hvert match, med farveoutput
- `grep -o "søgemønster" sti/til/fil` : søg efter linjer som matcher et mønster. Print kun den matchende tekst
- `cat sti/til/fil | grep -v "søgemønster"` : søg i `stdin` efter linjer der ikke matcher et mønster

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
