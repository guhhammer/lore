# fdp

Renderiza uma imagem de um gráfico de rede `force-directed` a partir de um arquivo `graphviz`.

## Usage
- `fdp -T png -O caminho/para/entrada.gv` : renderiza uma imagem PNG com um nome de arquivo baseado no nome do arquivo de entrada e formato de saída (-O maiúsculo)
- `fdp -T svg -o caminho/para/imagem.svg caminho/para/entrada.gv` : renderiza uma imagem SVG com o nome do arquivo de saída especificado (-o minúsculo)
- `fdp -T ps|pdf|svg|fig|png|gif|jpg|json|dot -O caminho/para/entrada.gv` : renderiza a saída nos formatos
- `echo "digraph {isso -> aquilo} " | fdp -T gif >caminho/para/imagem.gif` : renderiza uma imagem GIF usando `stdin` e `stdout`
- `fdp -?` : mostra ajuda

## Links
- More information: https://graphviz.org/doc/info/command.html
