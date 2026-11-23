# tail

Exibe a parte final de um arquivo.

## Usage
- `tail -n contagem caminho/para/arquivo` : mostra as últimas 'contagem' linhas de um arquivo
- `tail -n +contagem caminho/para/arquivo` : exibe um arquivo a partir de um número de linha específico
- `tail -c contagem caminho/para/arquivo` : mostra uma quantidade específica de bytes a partir do final de um arquivo
- `tail -f caminho/para/arquivo` : exibe as últimas linhas de um arquivo e continua lendo até pressionar `<Ctrl c>`
- `tail -F caminho/para/arquivo` : continua lendo um arquivo até `<Ctrl c>`, mesmo que ele fique inacessível
- `tail -n contagem -s segundos -f caminho/para/arquivo` : mostra as últimas 'num' linhas de um arquivo e atualiza a cada 'n' segundos

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html
