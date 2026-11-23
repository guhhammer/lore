# ag

The Silver Searcher. Parecido com o `ack`, mas com um foco em ser ainda mais rápido.

## Usage
- `ag foo` : acha arquivos que contém "foo" e imprime as linhas correspondentes no contexto
- `ag foo caminho/para/arquivo` : acha arquivos que contém "foo" em um diretório específico
- `ag -l foo` : acha arquivos que contém "foo", mas lista somente os nomes dos arquivos
- `ag -i -o FOO` : acha arquivos que contém "FOO" sem diferença de caixa e imprime somente o correspondente em vez de a linha inteira
- `ag foo -G bar` : acha "foo" em arquivos cujo nome corresponde a "bar"
- `ag '^ba(r|z)$'` : acha arquivos cujo conteúdo corresponde à expressão regular
- `ag -g foo` : acha arquivos cujo nome corresponde a "foo"

## Links
- More information: https://manned.org/ag
