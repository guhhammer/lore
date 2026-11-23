# git diff

Mostra le modifiche ai file tracciati.

## Usage
- `git diff` : mostra le modifiche non ancora nell'area di stage
- `git diff HEAD` : mostra tutte le modifiche non ancora salvate in un commit (incluse quelle nell'area di stage)
- `git diff --staged` : mostra solo le modifiche nell'area di stage (aggiunte, ma non ancora aggiunte ad un commit)
- `git diff 'HEAD@{3 months|weeks|days|hours|seconds ago}'` : mostra le modifiche di tutti i commit a partire da una certa data/ora (un'espressione temporale come ad esempio "1 week 2 days" o una data ISO)
- `git diff --stat commit` : mostra le differenze tramite delle statistiche come quali file modificati o l'istogramma e il totale delle righe inserite/cancellate
- `git diff --summary commit` : mostra un riepilogo dei file creati, rinominati o la cui modalità è cambiata a partire da un dato commit
- `git diff ramo_1..ramo_2 percorso/del/file` : confronta le versioni di un dato file tra due rami o commit
- `git diff ramo:percorso/del/file2 percorso/del/file1` : confronta le versioni di più file tra il ramo corrente e un altro ramo

## Links
- More information: https://git-scm.com/docs/git-diff
