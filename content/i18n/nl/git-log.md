# git log

Toon een geschiedenis van commits.

## Usage
- `git log` : toon de volgorde van commits, beginnend bij de huidige, in omgekeerd chronologische volgorde van de Git-repository in de huidige map
- `git log -p pad/naar/bestand_of_map` : toon de geschiedenis van een bepaald bestand of map, inclusief verschillen
- `git log --stat` : toon een overzicht van welke bestand(en) zijn gewijzigd in elke commit
- `git log --oneline --graph` : toon een grafiek van de commits in de huidige branch met alleen de eerste regel van elk commitbericht
- `git log --oneline --decorate --all --graph` : toon een grafiek van alle commits, tags en branches in de hele repository
- `git log -i --grep zoekstring` : toon alleen commits met berichten die een specifieke string bevatten, waarbij hoofdlettergebruik wordt genegeerd
- `git log -n n --author "auteur"` : toon de laatste `n` commits van een bepaalde auteur
- `git log --before "2017-01-29" --after "2017-01-17"` : toon commits tussen twee datums (yyyy-mm-dd)

## Links
- More information: https://git-scm.com/docs/git-log
