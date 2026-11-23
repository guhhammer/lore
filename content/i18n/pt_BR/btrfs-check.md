# btrfs check

Verifica ou repara um sistema de arquivos btrfs.

## Usage
- `sudo btrfs c caminho/para/partição` : verifica um sistema de arquivos btrfs
- `sudo btrfs c --repair caminho/para/partição` : verifica e repara um sistema de arquivos btrfs (perigoso)
- `sudo btrfs c -p caminho/para/partição` : mostra o andamento da verificação
- `sudo btrfs c --check-data-csum caminho/para/partição` : verifica a soma de verificação de cada bloco de dados (se o sistema de arquivos estiver bom)
- `sudo btrfs c -s n caminho/para/partição` : usa o `n`-ésimo superbloco (`n` pode ser 0, 1 ou 2)
- `sudo btrfs c --repair --init-csum-tree caminho/para/partição` : reconstrói a árvore de soma de verificação
- `sudo btrfs c --repair --init-extent-tree caminho/para/partição` : reconstrói a árvore de extensão

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-check.html
