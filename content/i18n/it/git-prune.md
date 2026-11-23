# git prune

Elimina dal database degli oggetti quelli non più raggiungibili. Questo comando è usato più spesso internamente da Git gc piuttosto che in modo diretto.

## Usage
- `git prune -n` : elenca quali oggetti saranno eliminati da Git prune senza eliminarli definitivamente
- `git prune -v` : elimina gli oggetti non raggiungibili e stampane un elenco su `stdout`
- `git prune --progress` : elimina gli oggetti non raggiungibili, mostrando lo stato di avanzamento

## Links
- More information: https://git-scm.com/docs/git-prune
