# btrfs scrub

Varre os sistemas de arquivos btrfs para verificar a integridade dos dados. Recomenda-se fazer uma varredura uma vez por mês.

## Usage
- `sudo btrfs sc start caminho/para/ponto_de_montagem_btrfs` : começar uma varredura
- `sudo btrfs sc status caminho/para/ponto_de_montagem_btrfs` : mostra o status de uma varredura em andamento ou concluída
- `sudo btrfs sc c caminho/para/ponto_de_montagem_btrfs` : cancela uma varredura em andamento
- `sudo btrfs sc r caminho/para/ponto_de_montagem_btrfs` : retoma uma varredura cancelada anteriormente
- `sudo btrfs sc start -B caminho/para/ponto_de_montagem_btrfs` : inicia uma varredura, mas espera até que a varredura termine antes de sair
- `sudo btrfs sc start -q caminho/para/ponto_de_montagem_btrfs` : inicia uma varredura no modo silencioso (não imprime erros ou estatísticas)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-scrub.html
