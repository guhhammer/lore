# btrfs rescue

Tenta recuperar um sistema de arquivos btrfs danificado.

## Usage
- `sudo btrfs resc ch caminho/para/partição` : reconstrói a árvore de metadados do sistema de arquivos (muito lento)
- `sudo btrfs resc fix-de caminho/para/partição` : corrige problemas relacionados ao alinhamento do tamanho do dispositivo (por exemplo, incapaz de montar o sistema de arquivos com incompatibilidade de super total de bytes)
- `sudo btrfs resc s caminho/para/partição` : recupera um superblock corrompido das cópias corretas (recupere a raiz da árvore do sistema de arquivos)
- `sudo btrfs resc z caminho/para/partição` : recupera-se de uma transação interrompida (corrige problemas de repetição de log)
- `sudo btrfs resc c` : cria um dispositivo de controle `/dev/btrfs-control` quando o `mknod` não estiver instalado

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-rescue.html
