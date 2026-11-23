# htop

Exibe informação dinâmica em tempo real acerca de processos em execução. Uma versão melhorada do comando `top`.

## Usage
- `htop` : inicializa `htop`
- `htop -u nome_usuário` : inicializa `htop` mostrando somente processos pertencentes a um usuário
- `htop -t` : apresenta os processos de forma hierárquica em uma visão de árvore para mostrar relações de pai-filho
- `htop -s item_de_ordenação` : ordena processos por um `item_de_ordenação` (utilize `htop --sort help` para ver as opções disponíveis)
- `htop -d 50` : inicializa `htop` com um atraso especificado entre atualizações, em décimos de segundo (p. ex. 50 = 5 segundos)
- `<?>` : vê comandos interativos enquanto roda htop
- `<Tab>` : muda para uma aba diferente
- `htop -h` : mostra ajuda

## Links
- More information: https://manned.org/htop
