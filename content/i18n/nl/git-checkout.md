# git checkout

Haal een branch of paden naar de working tree op.

## Usage
- `git checkout -b branch_naam` : creëer en schakel naar een nieuwe branch
- `git checkout -b branch_naam referentie` : creëer en schakel naar een nieuwe branch gebaseerd op een specifieke referentie (branch, remote/branch, tag zijn voorbeelden van geldige referenties)
- `git checkout branch_naam` : schakel over naar een bestaande lokale branch
- `git checkout -` : schakel over naar de eerder uitgecheckte branch
- `git checkout -t remote_naam/branch_naam` : schakel over naar een bestaande remote branch
- `git checkout .` : verwijder alle niet-toegevoegde wijzigingen in de huidige map (zie `git reset` voor meer ongedaan maken-achtige commando's)
- `git checkout pad/naar/bestand` : verwijder niet-toegevoegde wijzigingen aan een bepaald bestand
- `git checkout branch_naam -- pad/naar/bestand` : vervang een bestand in de huidige map door de versie die in een specifieke branch is vastgelegd

## Links
- More information: https://git-scm.com/docs/git-checkout
