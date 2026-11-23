# git blame

Afișează hash-ul commit-ului și ultimul autor pe fiecare linie a unui fișier.

## Usage
- `git blame cale/către/fișier` : tipărește fișierul cu numele autorului și hash-ul commit-ului pe fiecare linie
- `git blame -e cale/către/fișier` : tipărește fișierul cu email-ul autorului și hash-ul commit-ului pe fiecare linie
- `git blame commit cale/către/fișier` : tipărește fișierul cu numele autorului și hash-ul commit-ului pe fiecare linie la un commit specific
- `git blame commit~ cale/către/fișier` : tipărește fișierul cu numele autorului și hash-ul commit-ului pe fiecare linie înainte de un commit specific
- `git blame -L '/text/',+10 a82812aa^ cale/către/fișier` : sare la părintele unui commit specific și urmărește un text specific și 10 dintre liniile sale următoare
- `git blame -L linie_început,linie_sfârșit cale/către/fișier` : tipărește informațiile despre numele autorului și hash-ul commit-ului pentru un interval specific de linii
- `git blame -w -C -C -C cale/către/fișier` : ignoră spațiile albe și mutările de linii

## Links
- More information: https://git-scm.com/docs/git-blame
