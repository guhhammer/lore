# ansible

Gerencia grupos de computadores remotamente utilizando SSH. (use o arquivo `/etc/ansible/hosts` para adicionar novos grupos/hosts). Alguns subcomando como `ansible galaxy` possuis a própria documentação de uso.

## Usage
- `ansible grupo --list-hosts` : lista os hosts pertencentes a um grupo
- `ansible grupo -m ping` : realiza o ping de um grupo de hosts invocando o módulo ping
- `ansible grupo -m setup` : exibe fatos sobre um grupo de hosts invocando o módulo setup
- `ansible grupo -m command -a 'meu_comando'` : executa um comando em um grupo de hosts invocando o módulo command com argumentos
- `ansible grupo -b --ask-become-pass -m command -a 'meu_comando'` : executa um comando com privilégios administrativos
- `ansible grupo -i arquivo_inventario -m command -a 'meu_comando'` : executa um comando usando um arquivos de inventário customizado
- `ansible localhost -m debug -a 'var=groups.keys()'` : lista os grupos presentes em um inventário

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
