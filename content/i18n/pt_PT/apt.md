# apt

Gestor de pacotes das distribuições baseadas em Debian.

## Usage
- `sudo apt update` : actualiza a lista de pacotes disponíveis (recomenda-se executá-lo antes de outros comandos `apt`)
- `apt search criterio_de_pesquisa` : pesquisa pacotes correspondentes ao critério de pesquisa
- `apt show nome_do_pacote` : exibe as informações de um pacote
- `sudo apt install nome_do_pacote` : instala um pacote ou actualiza-o para a versão mais recente
- `sudo apt remove nome_do_pacote` : remove um pacote (Para remover os ficheiros de configuração deve-se usar a opção `purge` em vez de `remove`)
- `sudo apt upgrade` : actualiza os pacotes instalados para as versões mais recentes

## Links
- More information: https://manned.org/apt.8
