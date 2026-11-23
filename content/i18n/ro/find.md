# find

Găsește fișiere sau foldere într-un arbore de foldere, în mod recursiv.

## Usage
- `find cale/către/director -name '*.ext'` : găsește fișiere după extensie
- `find cale/către/director -path '*/cale/*/*.ext' -or -name '*tipar*'` : găsește fișiere care se potrivesc mai multor tipare de cale/nume
- `find cale/către/director -type d -iname '*lib*'` : găsește foldere care se potrivesc unui nume dat, în mod insensibil la majuscule
- `find cale/către/director -name '*.py' -not -path '*/site-packages/*'` : găsește fișiere care se potrivesc unui tipar dat, excluzând căi specifice
- `find cale/către/director -maxdepth 1 -size +500k -size -10M` : găsește fișiere într-un interval de dimensiuni dat, limitând adâncimea recursivă la „1”
- `find cale/către/director -name '*.ext' -exec wc -l {} \;` : rulează o comandă pentru fiecare fișier (folosește `{}` în comandă pentru a accesa numele fișierului)
- `find cale/către/director -daystart -mtime -1 -exec tar -cvf archive.tar {} \+` : găsește toate fișierele modificate astăzi și transmite rezultatele unei singure comenzi ca argumente
- `find cale/către/director -type f|d -empty -delete -print` : caută fișiere sau foldere goale și le șterge cu mesaje detaliate

## Links
- More information: https://manned.org/find
