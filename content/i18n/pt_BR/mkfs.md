# mkfs

Cria um sistema de arquivos Linux em uma partição do disco rígido. Esse comando está obsoleto em favor dos utilitários mkfs.<tipo> específicos de sistema de arquivos.

## Usage
- `sudo mkfs caminho/para/partição` : cria um sistema de arquivo ext2 do Linux em uma partição
- `sudo mkfs -t ext4 caminho/para/partição` : cria um sistema de arquivos de um tipo especificado
- `sudo mkfs -c -t ntfs caminho/para/partição` : cria um sistema de arquivos de um tipo especificado e verifica por blocos ruins

## Links
- More information: https://manned.org/mkfs
