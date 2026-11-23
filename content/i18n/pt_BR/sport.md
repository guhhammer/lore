# sport

Busque e instale Slackbuilds. Mais informações: <http://slackermedia.info/handbook/doku.php?id=slackbuilds>.

## Usage
- `sudo mkdir -p /usr/ports && sudo rsync -av rsync://slackbuilds.org /slackbuilds/$(awk '{print $2}' /etc/slackware-version)/ /usr/ports/` : puxa a lista de SlackBuilds para rodar `sport` pela primeira vez
- `sudo sport rsync` : puxa qualquer atualização para a árvore do sistema via `rsync`
- `sport search "palavra_chave"` : procura um pacote pelo nome
- `sport check pacote` : checa se um pacote está instalado
- `sport cat pacote` : exibe os arquivos README e `.info` de um pacote
- `sudo sport install pacote` : instala um pacote uma vez que as dependências estejam instaladas
- `sudo sport install $(<caminho/para/lista)` : instala uma lista de pacotes de um arquivo (formato: pacotes separados por espaço)
