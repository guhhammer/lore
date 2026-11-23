# git ls-remote

Elenca i riferimenti in un repository remoto dato un nome o un URL. Qualora né nome né URL siano specificati, il ramo predefinito è upstream - se configurato - oppure origin.

## Usage
- `git ls-remote` : mostra tutti i riferimenti nel repository remoto predefinito
- `git ls-remote --heads` : mostra solo i riferimenti HEAD nel repository remoto predefinito
- `git ls-remote -t` : mostra solo i riferimenti a tag nel repository remoto predefinito
- `git ls-remote url_repository` : mostra tutti i riferimenti da un repository remoto dato un nome o URL
- `git ls-remote nome_repository "criterio"` : filtra i riferimenti da un repository remoto rispetto a un dato criterio

## Links
- More information: https://git-scm.com/docs/git-ls-remote
