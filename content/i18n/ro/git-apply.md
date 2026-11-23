# git apply

Aplică un patch la fișiere și/sau la index fără a crea un commit.

## Usage
- `git apply -v cale/către/fișier` : afișează mesaje despre fișierele patch-uite
- `git apply --index cale/către/fișier` : aplică și adaugă fișierele patch-uite la index
- `curl -L https://exemplu.com/fișier.patch | git apply` : aplică un fișier patch de la distanță
- `git apply --stat --apply cale/către/fișier` : afișează diffstat pentru input și aplică patch-ul
- `git apply -R cale/către/fișier` : aplică patch-ul în sens invers
- `git apply --cached cale/către/fișier` : stochează rezultatul patch-ului în index fără a modifica working tree-ul

## Links
- More information: https://git-scm.com/docs/git-apply
