# git archive

Crea un archivio dei file nell'albero di lavoro.

## Usage
- `git archive -v HEAD` : crea un archivio `.tar` del contenuto in HEAD e stampa il risultato su `stdout`
- `git archive -v --format zip HEAD` : crea un archivio zip del contenuto in HEAD e stampa il risultato su `stdout`
- `git archive -v -o percorso/del/file.zip HEAD` : come sopra, ma scrivi l'archivio zip su file
- `git archive -o percorso/del/file.tar nome_ramo` : crea un archivio `.tar` dell'ultimo commit sul ramo specificato
- `git archive -o percorso/del/file.tar HEAD:percorso/della/directory` : crea un archivio `.tar` del contenuto di una specifica directory
- `git archive -o percorso/del/file.tar --prefix percorso/da/anteporre/ HEAD` : anteponi un percorso ad ogni file cosí da archiviarlo in una directory specifica

## Links
- More information: https://git-scm.com/docs/git-archive
