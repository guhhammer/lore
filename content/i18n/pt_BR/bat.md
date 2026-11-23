# bat

Imprime e concatena arquivos. Um clone do `cat` com realce de sintaxe e integração com Git.

## Usage
- `bat /caminho/para/arquivo` : imprime o conteúdo formatado de um arquivo para a saída padrão (`stdout`)
- `bat caminho/para/arquivo1 caminho/para/arquivo2 ... >/caminho/para/arquivo_destino` : concatena o conteúdo de vários arquivos em um arquivo destino
- `bat -pp caminho/para/arquivo` : remove estilizacão e desabilita páginação
- `bat -H 10|5:10|:10|10:|10:+5 caminho/para/arquivo` : destaca uma linha específica ou um intervalo de linhas com uma cor de fundo diferente
- `bat -A caminho/para/arquivo` : mostra caracteres não imprimíveis como espaço, tab ou nova linha
- `bat -n caminho/para/arquivo` : remove toda estilizacão exceto os números das linhas no arquivo de saída
- `bat -l json /caminho/para/arquivo.json` : realça a sintaxe de um arquivo ao definir explicitamente a linguagem (e.g. JSON)
- `bat -L` : mostra todas as linguagens suportadas

## Links
- More information: https://manned.org/bat
