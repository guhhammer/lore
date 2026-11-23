# git push

Invia i commit ad un repository remoto.

## Usage
- `git push` : invia le modifiche fatte nel ramo corrente locale al corrispondente ramo remoto
- `git push nome_repository_remoto nome_ramo` : invia le modifiche fatte in uno specifico ramo locale al corrispondente ramo remoto
- `git push -u nome_repository_remoto nome_ramo` : invia le modifiche fatte in uno specifico ramo locale al corrispondente ramo remoto ed imposta il ramo remoto come destinazione di default per i push/pull del ramo locale
- `git push nome_repository_remoto nome_ramo:nome_ramo_remoto` : invia le modifiche fatte in uno specifico ramo locale ad uno specifico ramo remoto
- `git push --all nome_repository_remoto` : invia le modifiche fatte in ogni ramo locale ai corrispondenti rami remoti in uno specifico repository remoto
- `git push nome_repository_remoto -d nome_ramo_remoto` : cancella un ramo di un repository remoto
- `git push --prune nome_repository_remoto` : cancella i rami remoti che non hanno un ramo locale corrispondente
- `git push --tags` : pubblica i tag che non sono già presenti nel repository remoto

## Links
- More information: https://git-scm.com/docs/git-push
