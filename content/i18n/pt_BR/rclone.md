# rclone

Copia, sincroniza ou move arquivos e diretórios de e para diversos serviços em nuvem.

## Usage
- `rclone config` : inicia um menu interativo para configurar o rclone
- `rclone lsf nome_do_remoto:caminho/para/diretorio` : lista o conteúdo de um diretório em um armazenamento remoto do rclone
- `rclone copy caminho/para/arquivo_ou_diretorio_origem nome_do_remoto:caminho/para/diretorio` : copia um arquivo ou diretório da máquina local para o destino remoto
- `rclone copy -i --max-age 24h nome_do_remoto:caminho/para/diretorio caminho/para/diretorio_local` : copia arquivos alterados nas últimas 24 horas da máquina local para um remoto, solicitando confirmação para cada arquivo
- `rclone sync caminho/para/arquivo_ou_diretorio nome_do_remoto:caminho/para/diretorio` : espelha um arquivo ou diretório específico (Nota: Ao contrário de copy, sync remove arquivos do remoto se eles não existirem localmente)
- `rclone -n delete nome_do_remoto:caminho/para/arquivo_ou_diretorio` : simula a exclusão de um arquivo ou diretório remoto (remova `--dry-run` do comando para realmente excluir)
- `rclone mount nome_do_remoto:caminho/para/diretorio caminho/para/ponto_de_montagem` : monta um armazenamento remoto do rclone (experimental)
- `fusermount -u caminho/para/ponto_de_montagem` : desmonta um armazenamento remoto do rclone se `<Ctrl c>` falhar (experimental)

## Links
- More information: https://rclone.org/commands/rclone/
