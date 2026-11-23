# ssh-add

Gerencia as chaves SSH carregadas no ssh-agent. Certifique-se de que o ssh-agent esteja em execução para que as chaves sejam carregadas nele.

## Usage
- `ssh-add` : adiciona as chaves SSH padrão em `~/.ssh` ao ssh-agent
- `ssh-add caminho/para/chave_privada` : adiciona uma chave específica ao ssh-agent
- `ssh-add -l` : lista as impressões digitais das chaves carregadas atualmente
- `ssh-add -d caminho/para/chave_privada` : exclui uma chave do ssh-agent
- `ssh-add -D` : exclui todas as chaves carregadas atualmente do ssh-agent
- `ssh-add -K caminho/para/chave_privada` : adiciona uma chave ao ssh-agent e ao keychain

## Links
- More information: https://man.openbsd.org/ssh-add
