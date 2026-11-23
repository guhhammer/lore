# git config

Gerencia configurações personalizadas para repositórios Git. Estas configurações podem ser locais (para o repositório atual) ou globais (para o usuário atual).

## Usage
- `git config --global user.name|user.email "Seu nome|e-mail@example.com"` : define globalmente seu nome ou e-mail (essas informações são necessárias para fazer commit em um repositório e serão incluídas em todos os commits)
- `git config -l --local|global` : lista configurações locais ou globais
- `git config -l --system --show-origin` : lista somente configurações do sistema (armazenadas no `/etc/gitconfig`), e exibe o local do arquivo
- `git config alias.unstage` : obtém o valor de uma dada configuração
- `git config --global alias.unstage "reset HEAD --"` : define o valor global de uma dada configuração
- `git config --global --unset alias.unstage` : reverte a configuração global para seu valor padrão
- `git config -e` : edita a configuração local do Git (`.git/config`) no editor padrão
- `git config --global -e` : edita a configuração global do Git (`~/.gitconfig` por padrão ou `$XDG_CONFIG_HOME/git/config` se tal arquivo existir) no editor padrão

## Links
- More information: https://git-scm.com/docs/git-config
