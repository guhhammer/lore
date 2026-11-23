# unzip

Ferramenta de descompactação de arquivos zip.

## Usage
- `unzip caminho/para/arquivo1.zip caminho/para/arquivo2.zip ...` : extrai arquivos zip
- `unzip caminho/para/arquivo1.zip caminho/para/arquivo2.zip ... -d caminho/para/saída` : extrai arquivos zip para caminhos específicos
- `unzip -c caminho/para/arquivo1.zip caminho/para/arquivo2.zip ...` : extrai arquivos/diretórios de arquivos para `stdout`
- `unzip -O gbk caminho/para/arquivo1.zip caminho/para/arquivo2.zip ...` : extrai o conteúdo do(s) arquivo(s) para `stdout` ao lado dos nomes dos arquivos extraídos
- `unzip -l caminho/para/arquivo.zip` : lista conteúdos de arquivos zip
- `unzip -j caminho/para/arquivo.zip caminho/para/arquivo1 caminho/para/arquivo2 ...` : extrai arquivos zip sem a estrutura dos diretórios

## Links
- More information: https://manned.org/unzip
