# history

Histórico de linha da comando.

## Usage
- `history` : exibe a lista de histórico de comandos com números de linha
- `history 20` : exibe os últimos 20 comandos (em Zsh ele exibe todos os comandos a partir do 20º)
- `history -d|f|i|E` : exibe histórico com data e hora em diferentes formatos (diponível apenas em Zsh)
- `history -c` : limpa a lista do histórico de comandos (apenas para o shell Bash atual)
- `history -w` : sobrescreve o arquivo de histórico com o histórico do shell Bash atual (frequentemente combinado com `history -c` para limpar o histórico)
- `history -d deslocamento` : deleta a entrada do histórico no deslocamento especificado

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-history
