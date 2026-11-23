# git subtree

Strumento per gestire le dipendenze di un progetto come progetti secondari.

## Usage
- `git subtree add -P percorso/della/directory --squash url_repository master` : aggiungi un repository Git come albero secondario
- `git subtree pull -P percorso/della/directory url_repository master` : aggiorna l'albero secondario di un repository al suo commit più recente
- `git subtree merge -P percorso/della/directory --squash url_repository master` : unisci un albero secondario al ramo principale (master)
- `git subtree push -P percorso/della/directory url_repository master` : invia commit all'albero secondario di un repository
- `git subtree split -P percorso/della/directory url_repository -b nome_ramo` : estrai la cronologia di un nuovo progetto dalla cronologia di un albero secondario

## Links
- More information: https://manned.org/git-subtree
