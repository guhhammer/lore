# bup

Sistema de backup baseado no formato Git packfile, oferecendo salvamentos incrementais e desduplicação global.

## Usage
- `bup -d caminho/para/repositório init` : inicializa um repositório de backup no diretório local especificado
- `bup -d caminho/para/repositório index caminho/para/diretório` : prepara um determinado diretório antes de fazer um backup
- `bup -d caminho/para/repositório save -n nome_do_backup caminho/para/diretório` : faz o backup de um diretório para o repositório
- `bup -d caminho/para/repositório ls` : exibe os snapshots de backup armazenados atualmente no repositório
- `bup -d caminho/para/repositório restore -C caminho/para/diretório_de_destino nome_do_backup` : restaura um snapshot de backup específico para um diretório de destino

## Links
- More information: https://manned.org/bup
