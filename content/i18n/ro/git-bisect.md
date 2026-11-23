# git bisect

Folosește căutarea binară pentru a găsi commit-ul care a introdus un bug. Git sare automat înainte și înapoi în graful de commit-uri pentru a restrânge progresiv commit-ul defect.

## Usage
- `git bisect start commit_defect commit_bun` : pornește o sesiune de bisect pe un interval de commit-uri delimitat de un commit cunoscut ca defect și unul cunoscut ca bun (de obicei mai vechi)
- `git bisect bun|defect` : pentru fiecare commit pe care `git bisect` îl selectează, marchează-l ca "bun" sau "defect" după testare pentru problema respectivă
- `git bisect reset` : termină sesiunea de bisect și revine la ramura anterioară
- `git bisect skip` : sare peste un commit în timpul unui bisect (de ex. unul care eșuează testele din cauza unei probleme diferite)
- `git bisect start commit_defect commit_bun -- cale/către/fișier_sau_director` : pornește o sesiune de bisect considerând doar commit-urile care modifică un fișier sau director specific
- `git bisect run cale/către/script_test argumente_script_opționale` : automatizează procesul de bisect folosind un script de test care iese cu 0 pentru "bun" și non-zero pentru "defect"
- `git bisect log` : afișează un jurnal al ceea ce s-a făcut până acum
- `git bisect visualize` : afișează commit-urile candidate rămase de verificat

## Links
- More information: https://git-scm.com/docs/git-bisect
