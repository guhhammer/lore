# du

Uso de disco: estima e sumariza o uso de espaço em disco de arquivos e diretórios.

## Usage
- `du -b|k|m caminho/para/diretório` : lista os tamanhos dos diretórios e qualquer subdiretório, em uma unidade de tamanho (B/KiB/MiB)
- `du -h caminho/para/diretório` : lista os tamanhos dos diretórios e subdiretórios, em tamanho legível por humanos (isto é seleciona automaticamente a unidade de tamanho apropriada para o tamanho)
- `du -sh caminho/para/diretório` : exibe o tamanho de um único diretório, em tamanho legível por humanos
- `du -ah caminho/para/diretório` : lista em tamanho legível por humanos todos os arquivos e diretórios dentro de um diretório
- `du -h -d N caminho/para/diretório` : lista em tamanho legível por humanos, até o nível N de profundidade um diretório e subdiretórios
- `du -ch */*.jpg` : lista em tamanho legível por humanos todos os arquivos `.jpg` dos subdiretórios do diretório atual, e exibe o total cumulativo no final
- `du -ah -t 1G|1024M|1048576K .[^.]* *` : lista todos os arquivos e diretórios (incluindo os ocultos) acima de um limite de tamanho (útil para invetigar o que está de fato ocupando o espaço)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html
