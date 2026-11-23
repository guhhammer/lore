# git submodule

Ispeziona, aggiorna e gestisce moduli secondari (submodule).

## Usage
- `git submodule update --init --recursive` : installa specifici moduli secondari di un repository
- `git submodule add url_repository` : aggiungi un repository Git come modulo secondario
- `git submodule add url_repository percorso/della/directory` : aggiungi un repository Git come modulo secondario alla directory specificata
- `git submodule foreach git pull` : aggiorna tutti i moduli secondari al loro commit più recente

## Links
- More information: https://git-scm.com/docs/git-submodule
