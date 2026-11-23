# git revert

Crea nuovi commit che invertano i risultati dei commit precedenti.

## Usage
- `git revert HEAD` : inverti il commit più recente
- `git revert HEAD~4` : inverti il quintùltimo commit
- `git revert nome_ramo~5..nome_ramo~2` : inverti più commit
- `git revert -n 0c01a9..9a1743` : inverti senza creare nuovi commit, ma modificando l'albero di lavoro

## Links
- More information: https://git-scm.com/docs/git-revert
