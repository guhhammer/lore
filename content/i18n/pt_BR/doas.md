# doas

Executa um comando como outro usuário.

## Usage
- `doas comando` : executa um comando como root
- `doas -u usuario comando` : executa um comando como outro usuário
- `doas -s` : executa o shell padrão como root
- `doas -C caminho/para/arquivo_de_configuração comando` : lê um arquivo de configuração e verifica se a execução de um comando como outro usuário é permitida
- `doas -L` : faz com que o `doas` solicite uma senha mesmo que já tenha sido inserida anteriormente

## Links
- More information: https://man.openbsd.org/doas
