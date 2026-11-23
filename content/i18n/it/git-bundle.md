# git bundle

Colloca oggetti e riferimenti in un archivio.

## Usage
- `git bundle create percorso/del/file.bundle nome_ramo` : crea un file bundle che contiene tutti gli oggetti e riferimenti di un dato ramo
- `git bundle create percorso/del/file.bundle --all` : crea un file bundle di tutti i rami
- `git bundle create percorso/del/file.bundle -5 HEAD` : crea un file bundle degli ultimi 5 commit sul ramo corrente
- `git bundle create percorso/del/file.bundle --since=7.days HEAD` : crea un file bundle degli ultimi 7 giorni
- `git bundle verify percorso/del/file.bundle` : verifica che un file bundle sia valido e possa essere applicato al repository in uso
- `git bundle unbundle percorso/del/file.bundle` : stampa su `stdout` la lista di riferimenti contenuti in un bundle
- `git pull percorso/del/file.bundle nome_ramo` : dato un file bundle, estrai un ramo specifico nel repository in uso

## Links
- More information: https://git-scm.com/docs/git-bundle
