# git merge

Voeg branches samen.

## Usage
- `git merge branch_naam1 branch_naam2 ...` : voeg branches samen met de huidige branch
- `git merge -e branch_naam` : bewerk het samenvoegingsbericht
- `git merge --no-ff branch_naam` : voeg een branch samen en maak een samenvoegingscommit
- `git merge --squash branch_naam` : voeg het resultaat van het samenvoegen van een branch toe zonder een commit aan te maken
- `git merge --abort` : breek een samenvoeging af in geval van conflicten
- `git merge -s strategie -X strategie_optie branch_naam` : voeg samen met behulp van een specifieke strategie

## Links
- More information: https://git-scm.com/docs/git-merge
