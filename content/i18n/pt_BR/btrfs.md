# btrfs

Um sistema de arquivos baseado no princípio copy-on-write (COW) para Linux. Alguns subcomandos como `device` têm sua própria documentação de uso.

## Usage
- `sudo btrfs su c caminho/para/subvolume` : cria subvolume
- `sudo btrfs su l caminho/para/ponto_de_montagem` : lista subvolumes
- `sudo btrfs f df caminho/para/ponto_de_montagem` : mostra informações de uso do espaço
- `sudo btrfs qu e caminho/para/subvolume` : ativa a cota
- `sudo btrfs qg s caminho/para/subvolume` : mostra a cota

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs.html
