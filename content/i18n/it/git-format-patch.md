# git format-patch

Prepara file `.patch`. Utile per l'invio di commit via email.

## Usage
- `git format-patch origin` : crea un file `.patch` (il nome è assegnato automaticamente) con i commit non ancora inviati al repository remoto
- `git format-patch commit_1..commit_2` : scrivi su `stdout` un file `.patch` per l'intervallo di commit definito dai due commit dati
- `git format-patch -3` : scrivi un file `.patch` per gli ultimi 3 commit

## Links
- More information: https://git-scm.com/docs/git-format-patch
