# btrfs property

Obtém, define ou lista propriedades para um determinado objeto de sistema de arquivos btrfs (arquivos, diretórios, subvolumes, sistemas de arquivos ou dispositivos).

## Usage
- `sudo btrfs p l caminho/para/objeto_btrfs` : lista as propriedades disponíveis (e descrições) para o objeto btrfs fornecido
- `sudo btrfs p g caminho/para/objeto_btrfs` : obtém todas as propriedades para o objeto btrfs fornecido
- `sudo btrfs p g caminho/para/sistema_de_arquivos_btrfs label` : obtém a propriedade `label` (etiqueta) para o sistema de arquivos ou dispositivo btrfs fornecido
- `sudo btrfs p g -t subvol|filesystem|inode|device caminho/para/sistema_de_arquivos_btrfs` : obtém todas as propriedades específicas do tipo de objeto para o sistema de arquivos ou dispositivo btrfs fornecido
- `sudo btrfs p s caminho/para/inode_btrfs compression zstd|zlib|lzo|none` : define a propriedade de `compression` (compactação) para um determinado inode btrfs (um arquivo ou diretório)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-property.html
