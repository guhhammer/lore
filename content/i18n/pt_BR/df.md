# df

Exibe uma visão geral do uso de espaço de disco do sistema de arquivos.

## Usage
- `df` : exibe todos os sistemas de arquivos e seu uso de disco usando unidades 512-bytes
- `df -h -c` : usa unidades legíveis para humanos (baseadas em potências de 1024) e exibe um total
- `df -H` : usa unidades legíveis para humanos (baseadas em potências de 1000)
- `df caminho/para/arquivo_ou_diretório` : exibe o sistema de arquivos e seu uso do disco contendo o arquivo ou diretório dado
- `df -iT` : inclui estatísticas do número de nós livres e usados incluindo Tipos do sistema de arquivos
- `df -k` : usa unidades 1024-bytes ao escrever figuras de espaço
- `df -P` : exibe informação em uma maneira portátil

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=df
