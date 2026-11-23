# git describe

Rendi il nome di un oggetto Git più leggibile usando i riferimenti disponibili.

## Usage
- `git describe` : crea un nome univoco per il commit corrente (il nome contiene i tag più recenti, il numero di commit aggiuntivi, e l'hash breve del commit)
- `git describe --abbrev=4` : crea un nome di 4 cifre per l'hash breve del commit
- `git describe --all` : genera un nome che includa anche il percorso di riferimento
- `git describe v1.0.0` : descrivi un tag Git
- `git describe nome_ramo` : crea un nome per l'ultimo commit di un dato ramo

## Links
- More information: https://git-scm.com/docs/git-describe
