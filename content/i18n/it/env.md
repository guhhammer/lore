# env

Mostra le variabili d'ambiente o esegui un programma in un ambiente modificato.

## Usage
- `env` : mostra le variabili d'ambiente
- `env programma` : esegui un programma. Utilizzato spesso in script dopo lo shebang (#!) per cercare il percorso del programma
- `env -i programma` : resetta l'ambiente ed esegui un programma
- `env -u variabile programma` : rimuovi una variabile dall'ambiente ed esegui un programma
- `env variabile=valore programma` : setta una variabile ed esegui un programma

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/env-invocation.html
