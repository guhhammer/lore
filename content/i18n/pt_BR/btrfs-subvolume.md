# btrfs subvolume

Gerencia subvolumes e snapshots btrfs.

## Usage
- `sudo btrfs su c caminho/para/novo_subvolume` : cria um novo subvolume vazio
- `sudo btrfs su l caminho/para/sistema_de_arquivos_btrfs` : lista todos os subvolumes e snapshots no sistema de arquivos especificado
- `sudo btrfs su d caminho/para/subvolume` : exclui um subvolume
- `sudo btrfs su sn -r caminho/para/subvolume_de_origem caminho/para/destino` : cria um snapshot somente leitura de um subvolume existente
- `sudo btrfs su sn caminho/para/subvolume_de_origem caminho/para/destino` : cria um snapshot de leitura/gravação de um subvolume existente
- `sudo btrfs su sh caminho/para/subvolume` : mostra informações detalhadas sobre um subvolume

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-subvolume.html
