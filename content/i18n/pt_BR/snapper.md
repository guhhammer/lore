# snapper

Ferramenta de gerenciamento de snapshots do sistema de arquivos. Mais informações: <http://snapper.io/manpages/snapper.html>.

## Usage
- `snapper list-configs` : lista configurações de snapshots
- `snapper -c configuração create-config caminho/para/diretório` : cria configuração do snapper
- `snapper -c configuração create -d "descrição_do_snapshot"` : cria um snapshot com uma descrição
- `snapper -c configuração list` : lista snapshots para uma configuração
- `snapper -c configuração delete número_do_snapshot` : exclue um snapshot
- `snapper -c configuração delete snapshot_X-snapshot_Y` : exclue um intervalo de snapshots
