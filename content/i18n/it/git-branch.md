# git branch

Il principale comando Git per lavorare con i rami.

## Usage
- `git branch` : elenca i rami locali. Il ramo corrente è evidenziato da un `*`
- `git branch -a` : elenca tutti i rami (locali e remoti)
- `git branch nome_ramo` : crea un nuovo ramo a partire dal commit corrente
- `git branch nome_ramo hash_commit` : crea un nuovo ramo a partire dal commit specificato
- `git branch -m vecchio_nome nuovo_nome` : rinomina un ramo (non applicabile sul ramo corrente)
- `git branch -d nome_ramo` : cancella un ramo locale (non applicabile sul ramo corrente)

## Links
- More information: https://git-scm.com/docs/git-branch
