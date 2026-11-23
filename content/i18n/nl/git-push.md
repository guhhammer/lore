# git push

Push commits naar een externe repository.

## Usage
- `git push` : stuur lokale aanpassingen in de huidige branch naar de standaard externe tegenhanger
- `git push externe_naam lokale_branch` : stuur aanpassingen van een specifieke lokale branch naar zijn externe tegenhanger
- `git push -u externe_naam lokale_branch` : stuur veranderingen van een specifieke lokale branch naar zijn externe tegenhanger en stel de externe branch in als het standaard push/pull-doel voor de lokale branch
- `git push externe_naam lokale_branch:externe_branch` : stuur veranderingen van een specifieke lokale branch naar een specifieke externe branch
- `git push --all externe_naam` : stuur aanpassingen op alle lokale branches naar hun tegenhangers in de opgegeven externe repository
- `git push externe_naam -d externe_branch` : verwijder een branch in een externe repository
- `git push --prune externe_naam` : verwijder externe branches die geen lokale tegenhanger hebben
- `git push --tags` : publiceer tags die nog niet in de externe repository staan

## Links
- More information: https://git-scm.com/docs/git-push
