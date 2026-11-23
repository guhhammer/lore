# rsync

Transfira arquivos para ou de um host remote (mas não entre dois hosts remotos), usando SSH por padrão. Para especificar um caminho remoto, use `host:caminho/para/arquivo_ou_diretório`.

## Usage
- `rsync caminho/para/origem caminho/para/destino` : transfere um arquivo
- `rsync -a caminho/para/origem caminho/para/destino` : usa o modo de arquivo (copia recursivamente diretórios, copia links simbólicos sem resolver e preserva permissões, propriedade e tempos de modificação)
- `rsync -zvhP caminho/para/origem caminho/para/destino` : comprime os dados à medida que são enviados ao destino, exibe progresso detalhado e legível, e mantém arquivos parcialmente transferidos se forem interrompidos
- `rsync -r caminho/para/origem caminho/para/destino` : copia recursivamente diretórios
- `rsync -r caminho/para/origem/ caminho/para/destino` : transfere os conteúdos do diretório, mas não o diretório em si
- `rsync -auL caminho/para/origem caminho/para/destino` : copia diretórios, usa o modo de arquivamento, resolve links simbólicos e ignora arquivos que são mais recentes no destino
- `rsync -r --delete rsync://host:caminho/para/origem caminho/para/destino` : transfere um diretório para um host remoto executando o `rsyncd` and exclui arquivos no destino que não existem na origem
- `rsync -e 'ssh -p porta' --info=progress2 host:caminho/para/origem caminho/para/destino` : transfere um arquivo por SSH usando uma porta diferente da padrão (22) e mostra o progresso global

## Links
- More information: https://download.samba.org/pub/rsync/rsync.1
