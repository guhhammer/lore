# git diff

Toon veranderingen in bijgehouden bestanden.

## Usage
- `git diff` : toon niet-toegevoegde veranderingen
- `git diff HEAD` : toon alle niet-gecommitte (inclusief de toegevoegde) veranderingen
- `git diff --staged` : toon alleen toegevoegde (nog niet gecommitte) veranderingen
- `git diff 'HEAD@{3 months|weeks|days|hours|seconds ago}'` : toon veranderingen van alle commits sinds een bepaalde datum/tijd (een datumexpressie, b.v. "1 week 2 days" of een ISO-datum)
- `git diff --stat commit` : toon diff-statistieken, zoals gewijzigde bestanden, histogram en het totale aantal ingevoegde/verwijderde regels
- `git diff --summary commit` : toon een samenvatting van aangemaakte bestanden, hernoemingen en moduswijzigingen sinds een bepaalde commit
- `git diff branch_1..branch_2 pad/naar/bestand` : vergelijk een bestand tussen twee branches of commits
- `git diff andere_branch:pad/naar/bestand2 pad/naar/bestand1` : vergelijk verschillende bestanden van de huidige branch met een andere branch

## Links
- More information: https://git-scm.com/docs/git-diff
