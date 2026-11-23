# btrfs filesystem

Gerencia sistemas de arquivos btrfs.

## Usage
- `btrfs filesystem usage caminho/para/montagem_btrfs` : mostra uso do sistema de arquivos (opcionalmente execute como root para mostrar informações detalhadas)
- `sudo btrfs filesystem show caminho/para/montagem_btrfs` : mostra uso por dispositivos individuais
- `sudo btrfs filesystem defragment -v caminho/para/arquivo` : desfragmenta um único arquivo em um sistema de arquivos btrfs (evite enquanto um agente de desduplicação estiver em execução)
- `sudo btrfs filesystem defragment -v -r caminho/para/diretório` : desfragmenta um diretório recursivamente (não cruza os limites do subvolume)
- `sudo btrfs filesystem sync caminho/para/montagem_btrfs` : força a sincronização de blocos de dados não gravados com o(s) disco(s)
- `sudo btrfs filesystem du --summarize caminho/para/diretório` : resume o uso do disco para os arquivos em um diretório recursivamente

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-filesystem.html
