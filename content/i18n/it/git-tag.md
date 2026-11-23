# git tag

Crea, elenca, cancella o verifica tag. Un tag è un riferimento statico a uno specifico commit.

## Usage
- `git tag` : mostra tutti i tag
- `git tag nome_tag` : crea un tag con un nome, puntandolo al commit corrente
- `git tag nome_tag commit` : crea un tag con un nome, puntandolo ad un dato commit
- `git tag nome_tag -m messaggio_tag` : crea un tag annotandolo con un messaggio
- `git tag -d nome_tag` : cancella un tag, dato il suo nome
- `git fetch -t` : scarica tag aggiornati da upstream
- `git tag --contains commit` : mostra tutti i tag i cui predecessori includono uno specifico commit

## Links
- More information: https://git-scm.com/docs/git-tag
