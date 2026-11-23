# git log

Mostra la cronologia dei commit.

## Usage
- `git log` : mostra la sequenza dei commit del ramo del repository in uso, a partire dal commit corrente e andando in ordine cronologico inverso
- `git log -p percorso/del/file_o_directory` : mostra la cronologia di un dato file o directory, mostrando anche le modifiche
- `git log --stat` : offri una panoramica dei file che sono cambiati ad ogni commit
- `git log --oneline --graph` : mostra il grafo dei commit nel ramo corrente, includendo solo la prima riga di ogni messaggio di commit
- `git log --oneline --decorate --all --graph` : mostra il grafo di tutti i commit, tag e rami dell'intero repository
- `git log -i --grep stringa_da_cercare` : mostra solo i commit il cui messaggio contiene una data stringa (ignorando maiuscole/minuscole)
- `git log -n numero --author "autore"` : mostra gli ultimi N commit di un certo autore
- `git log --before "2017-01-29" --after "2017-01-17"` : mostra i commit effettuati tra due date (yyyy-mm-dd)

## Links
- More information: https://git-scm.com/docs/git-log
