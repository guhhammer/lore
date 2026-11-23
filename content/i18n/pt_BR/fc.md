# fc

Abre o último comando executado em um editor de texto.

## Usage
- `fc` : abre o último comando executado no editor de texto padrão do sistema
- `fc -e 'emacs'` : especifica o editor de texto que será utilizado ao executar o comando
- `fc -l` : exibe um histórico dos últimos comandos executados
- `fc -l -r` : lista os comandos recentes em ordem reversa
- `fc número` : edita e executa um comando do histórico
- `fc '416' '420'` : edita comandos em um dado intervalo e executa-os
- `fc --help` : mosta ajuda

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-fc
