# git stash

Sla lokale Git-wijzigingen op in een tijdelijke locatie.

## Usage
- `git stash` : sla de huidige niet-gecommitte veranderingen op
- `git stash -u` : sla huidige veranderingen op, inclusief nieuwe niet-bijgehouden bestanden
- `git stash -p` : selecteer interactief delen van gewijzigde bestanden om op te slaan
- `git stash list` : toon alle stashes
- `git stash show -p` : toon de wijzigingen als een patch tussen de stash en de commit van toen de stash voor het eerst werd aangemaakt
- `git stash pop` : pas een stash toe en verwijder deze van de stash-lijst als het toepassen niet tot conflicten leidt
- `git stash drop` : verwijder de recentste stash
- `git stash clear` : verwijder alle stashes

## Links
- More information: https://git-scm.com/docs/git-stash
