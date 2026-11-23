# pacman-deptest

Verifica cada dependência especificada e retorna uma lista de dependências que não estão satisfeitas atualmente no sistema.

## Usage
- `pacman -T pacote1 pacote2 ...` : imprime os nomes de pacotes das dependências que não estão instaladas
- `pacman -T "bash>=5"` : verifica se o pacote instalado satisfaz a versão mínima dada
- `pacman -T "bash>5"` : verifica se uma versão posterior de um pacote está instalado
- `pacman -Th` : exibe ajuda

## Links
- More information: https://manned.org/pacman.8
