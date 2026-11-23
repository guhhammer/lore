# git switch

Wissel tussen Git branches. Vereist Git-versie 2.23+.

## Usage
- `git switch branch_naam` : wissel naar een bestaande branch
- `git switch -c branch_naam` : creëer een nieuwe branch en wissel ernaar
- `git switch -c branch_naam commit` : creëer een nieuwe branch gebaseerd op een bestaande commit en wissel ernaar
- `git switch -` : wissel naar de vorige branch
- `git switch --recurse-submodules branch_naam` : wissel naar een branch en update alle submodules zodat ze overeenkomen
- `git switch -m branch_naam` : wissel naar een branch en voeg automatisch de huidige branch en alle niet-vastgelegde aanpassingen hierin samen
- `git switch -d tag|commit` : wissel naar een tag of commit
- `git switch -t externe_naam/branch` : wissel naar een branch van een andere externe repository

## Links
- More information: https://git-scm.com/docs/git-switch
