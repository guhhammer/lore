# git checkout

Cambia rami o ripristina i file dell'albero di lavoro.

## Usage
- `git checkout -b nome_ramo` : crea e passa ad un nuovo ramo
- `git checkout -b nome_ramo riferimento` : crea e passa ad un nuovo ramo a partire dal riferimento specificato (ramo-locale, remote/ramo-remoto, tag sono alcuni esempi di riferimenti validi)
- `git checkout nome_ramo` : passa ad un ramo locale esistente
- `git checkout -t nome_repository_remoto/nome_ramo` : passa ad un ramo remoto esistente
- `git checkout .` : annulla tutte le modifiche nella directory corrente che non sono state aggiunte all'area di stage (vedi `git reset` per più comandi simili)
- `git checkout nome_file` : annulla tutte le modifiche di un dato file non aggiunte all'area di stage
- `git checkout nome_ramo -- nome_file` : sostituisci un file con il contenuto del suo corrispondente localizzato su un altro ramo

## Links
- More information: https://git-scm.com/docs/git-checkout
