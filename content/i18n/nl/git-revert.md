# git revert

Maak nieuwe commits aan die eerdere commits ongedaan maken.

## Usage
- `git revert HEAD` : maak de laatste commit ongedaan
- `git revert HEAD~4` : maak de vijfde laatste commit ongedaan
- `git revert 0c01a9` : maak een specifieke commit ongedaan
- `git revert branch_naam~5..branch_naam~2` : maak meerdere commits ongedaan
- `git revert -n 0c01a9..9a1743` : maak geen nieuwe commits aan, pas alleen de working tree aan
- `git revert --abort` : annuleer een Git revert na een merge conflict

## Links
- More information: https://git-scm.com/docs/git-revert
