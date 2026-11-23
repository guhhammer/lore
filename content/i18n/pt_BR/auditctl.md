# auditctl

Utilitário para controlar o comportamento, obter o status e gerenciar regras do Sistema de Auditoria do Linux.

## Usage
- `sudo auditctl -s` : exibe o estatus do sistema de auditoria
- `sudo auditctl -l` : lista todas as regras de auditoria atualmente carregadas
- `sudo auditctl -D` : deleta todas as regras de auditoria
- `sudo auditctl -e 1|0` : habilita ou desabilita o sistema de auditoria
- `sudo auditctl -a always,exit -F arch=b64 -F path=/caminho/para/arquivo -F perm=wa` : monitora um arquivo em busca de alterações
- `sudo auditctl -a always,exit -F arch=b64 -F dir=/caminho/para/diretorio/ -F perm=wa` : monitora recursivamente um diretório em busca de alterações
- `auditctl -h` : mostra ajuda

## Links
- More information: https://manned.org/auditctl
