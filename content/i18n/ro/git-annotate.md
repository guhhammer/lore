# git annotate

Afișează hash-ul commit-ului și ultimul autor pe fiecare linie a unui fișier. Vezi `git blame`, care este preferat față de `git annotate`. `git annotate` este oferit pentru cei familiarizați cu alte sisteme de control al versiunilor.

## Usage
- `git annotate cale/către/fișier` : afișează un fișier cu numele autorului și hash-ul commit-ului adăugat la începutul fiecărei linii
- `git annotate -e cale/către/fișier` : afișează un fișier cu email-ul autorului și hash-ul commit-ului adăugat la începutul fiecărei linii
- `git annotate -L :regex cale/către/fișier` : afișează doar rândurile care se potrivesc cu un `regex`

## Links
- More information: https://git-scm.com/docs/git-annotate
