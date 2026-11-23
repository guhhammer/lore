# zsh

Z SHell, um interpretador de linha de comando compatível com o Bash.

## Usage
- `zsh` : inicie uma sessão shell interativa
- `zsh -c "echo Olá Mundo"` : execute comandos específicos
- `zsh caminho/para/script.zsh` : execute um script específico
- `zsh --no-exec caminho/para/script.zsh` : verifica um script específico por erros de sintaxe sem executá-lo
- `echo Olá Mundo | zsh` : executa comandos específicos da `stdin`
- `zsh --xtrace caminho/para/script.zsh` : execute um script específico, imprimindo cada comando do script antes de executá-lo
- `zsh --verbose` : inicie uma sessão shell interativa no modo verboso, imprimindo cada comando antes de executá-lo
- `noglob comando` : executa um comando específico dentro do Zsh com padrões `glob` desativados

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Invocation.html#Invocation
