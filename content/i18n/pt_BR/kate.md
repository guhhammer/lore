# kate

Editor de texto avançado do KDE.

## Usage
- `kate caminho/para/arquivo1 caminho/para/arquivo2 ...` : abre arquivos específicos
- `kate https://example.com/caminho/para/arquivo1 https://example.com/caminho/para/arquivo2 ...` : abre arquivos remotos específicos
- `kate -n` : cria uma uma nova instância do editor mesmo que uma já esteja aberta
- `kate -l número_linha caminho/para/arquivo` : abre um arquivo com o cursor em uma linha específica
- `kate -l número_linha -c número_coluna caminho/para/arquivo` : abre um arquivo com o cursor em uma linha e coluna específica
- `cat caminho/para/arquivo | kate -i` : cria um arquivo a partir do `stdin`
- `kate -h` : mostra ajuda
