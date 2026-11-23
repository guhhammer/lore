# btrfs restore

Tenta salvar arquivos de um sistema de arquivos btrfs danificado.

## Usage
- `sudo btrfs rest caminho/para/dispositivo_btrfs caminho/para/diretório_alvo` : restaura todos os arquivos de um sistema de arquivos btrfs para um determinado diretório
- `sudo btrfs rest -D caminho/para/dispositivo_btrfs caminho/para/diretório_alvo` : lista (sem escrever) os arquivos a serem restaurados de um sistema de arquivos btrfs
- `sudo btrfs rest --path-regex regex -c caminho/para/dispositivo_btrfs caminho/para/diretório_alvo` : restaura arquivos correspondentes a determinados padrões `regex` (case-insensitive) de um sistema de arquivos btrfs (todos os diretórios pai do(s) arquivo(s) de destino também devem corresponder)
- `sudo btrfs rest -t bytenr caminho/para/dispositivo_btrfs caminho/para/diretório_alvo` : restaura arquivos de um sistema de arquivos btrfs usando um `bytenr` específico da árvore raiz (consulte `btrfs-find-root`)
- `sudo btrfs rest -m -x -S -o caminho/para/dispositivo_btrfs caminho/para/diretório_alvo` : restaura arquivos de um sistema de arquivos btrfs (juntamente com metadados, atributos estendidos e Symlinks), sobrescrevendo arquivos no destino

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-restore.html
