# git reset

Desfaz os commits ou as alterações nào preparadas, redefinindo o Git HEAD atual para o estado especificado. Se um caminho é passado, funcionará como "não preparado"; se um hash de commit ou uma branch é passado, funcionará como "sem commit".

## Usage
- `git reset` : remove tudo da preparação
- `git reset caminho/para/arquivo1 caminho/para/arquivo2 ...` : remove arquivo(s) específico(s) da preparação
- `git reset -p caminho/para/arquivo` : interativamente remove partes de um arquivo da preparação
- `git reset HEAD~` : desfaz o último commit, mantendo suas alterações (e quaisquer outras alteração não confirmadas) no sistema de arquivos
- `git reset --soft HEAD~2` : desfaz os últimos dois commits, adicionando suas alterações na área de preparação, isso é, preparando-os para o commit
- `git reset --hard` : descarta quaisquer alterações sem commit, preparadas ou não (para apenas alterações não preparadas, use o `git checkout`)
- `git reset --hard commit` : redefine o repositório para um determinado commit, descartando as alterações com commit, preparadas e sem commit desde então

## Links
- More information: https://git-scm.com/docs/git-reset
