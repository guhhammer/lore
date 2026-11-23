# tar

Ferramenta de compressão de arquivos. Utilizado com metodos de compressão como o de gzip ou bzip2.

## Usage
- `tar cf caminho/para/alvo.tar caminho/para/arquivo1 caminho/para/arquivo2 ...` : cria um arquivo compactado e o escreve para um arquivo
- `tar czf alvo.tar.gz caminho/para/arquivo1 caminho/para/arquivo2 ...` : cria um arquivo gzip e o escreve para um arquivo
- `tar czf caminho/para/alvo.tar.gz -C caminho/para/diretório .` : cria um arquivo gzip a partir de um diretório usando caminhos relativos
- `tar xvf origem.tar.ext` : extrai arquivos de um arquivo (compactado)
- `tar xf caminho/para/origem.tar.ext -C caminho/para/diretório` : extrai um arquivo (compactado) para um diretório alvo
- `tar caf caminho/para/alvo.tar.xz caminho/para/arquivo1 caminho/para/arquivo2 ...` : cria um arquivo compactado e o escreve para um arquivo, usando a extensão de arquivo para determinar automaticamente a compressão do programa
- `tar tvf input.tar` : lista arquivos de um arquivo tar
- `tar xf caminho/para/alvo.tar --wildcards "*.html"` : extrai arquivos que correspondam a um padrão de um arquivo compactado

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
