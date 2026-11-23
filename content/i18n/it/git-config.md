# git config

Configura le impostazioni di uno o piu repository Git. Le configurazioni possono essere sia locali (per il repository corrente) che globali (per l'utente corrente).

## Usage
- `git config -l --local` : elenca solo le opzioni di configurazione locali (salvate in `.git/config` nel repository corrente)
- `git config -l --global` : elenca solo le opzioni di configurazione globali (salvate in `~/.gitconfig`)
- `git config -l` : elenca tutte le opzioni di configurazione impostate, sia locali che globali
- `git config alias.unstage` : mostra il valore di una data opzione di configurazione
- `git config --global alias.unstage "reset HEAD --"` : imposta il valore globale di una data opzione di configurazione
- `git config --global --unset alias.unstage` : ripristina una opzione di configurazione globale al suo valore di default

## Links
- More information: https://git-scm.com/docs/git-config
