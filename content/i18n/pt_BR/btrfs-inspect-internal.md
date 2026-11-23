# btrfs inspect-internal

Consulta informações internas de um sistema de arquivos btrfs.

## Usage
- `sudo btrfs i dump-s caminho/para/partição` : imprime informações de superblocos
- `sudo btrfs i dump-s -a caminho/para/partição` : imprime as informações do superbloco e de todas as suas cópias
- `sudo btrfs i dump-t caminho/para/partição` : imprime informações de metadados do sistema de arquivos
- `sudo btrfs i i n caminho/para/montagem_btrfs` : imprime lista de arquivos no `n`-ésimo inode
- `sudo btrfs i lo endereço_lógico caminho/para/montagem_btrfs` : imprime a lista de arquivos em um determinado endereço lógico
- `sudo btrfs i t caminho/para/partição` : imprime as estatísticas das árvores raiz, extensão, csum e fs

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-inspect-internal.html
