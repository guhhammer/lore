# pacdiff

Utilitário de manutenção para arquivos `.pacorig`, `.pacnew` e `.pacsave` criados pelo `pacman`.

## Usage
- `pacdiff` : reveja arquivos que precisam manutenção em modo interativo
- `pacdiff -s` : usa `sudo` e `sudoedit` para remover e mesclar arquivos
- `pacdiff -s -b` : reveja arquivos precisando de manutenção, criando `.bak`ups do original se você `s(O)brescrever`
- `DIFFPROG=editor pacdiff` : usa um editor específico para ver e mesclar arquivos de configuração (o padrão é `vim -d`)
- `pacdiff -l` : procura arquivos de configuração com `locate` ao invés de usar o banco de dados do `pacman`
- `pacdiff -h` : mostra ajuda

## Links
- More information: https://manned.org/pacdiff
