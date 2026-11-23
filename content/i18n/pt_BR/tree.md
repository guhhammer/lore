# tree

Exibe o conteúdo do diretório atual em formato de árvore.

## Usage
- `tree -L num` : exibe os arquivos e diretórios de acordo com o nível de profundidade 'num' informado (onde 1 significa o diretório atual)
- `tree -d` : exibe apenas diretórios
- `tree -a -C` : inclui a exibição de arquivos ocultos com colorização diferenciada
- `tree -i -f` : exibe a árvore sem identação, mostrando o caminho completo (usar `-N` para não escapar espaços em branco e caracteres especiais)
- `tree -s -h --du` : exibe o tamanho de cada arquivo e o tamanho acumulado de cada diretório, em um formato de leitura para humanos
- `tree -P '*.txt' --prune` : exibe arquivos em uma árvore hierárquica, utilizando um padrão coringa, e eliminando diretórios que não contêm arquivos correspondentes ao informado
- `tree -P nome_diretorio --matchdirs --prune` : exibe diretórios em uma árvore hierárquica, utilizando um padrão coringa, e eliminando diretórios que não possuem ancestrais do informado
- `tree -I 'nome_diretorio1|nome_diretorio2'` : exibe a árvore ignorando os diretórios informados

## Links
- More information: https://manned.org/tree
