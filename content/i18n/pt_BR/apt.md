# apt

Utilitário de gerenciamento de pacotes de distribuições baseadas em Debian. Substituto recomendado para `apt-get` quando usado de forma interativa em versões do Ubuntu mais novas que 16.04.

## Usage
- `sudo apt update` : atualiza a lista de pacotes e versões disponíveis (recomenda-se executá-lo antes de outros comandos `apt`)
- `apt search pacote` : busca por um determinado pacote
- `apt show pacote` : exibe as informações de um pacote
- `sudo apt install pacote` : instala um pacote ou atualiza-o para a versão mais recente
- `sudo apt remove pacote` : remove um pacote (para remover também os arquivos de configuração deve-se usar a opção `purge` ao invés do `remove`)
- `sudo apt upgrade` : atualiza todos os pacotes instalados para suas versões mais recentes
- `apt list` : lista todos os pacotes
- `apt list -i` : lista todos os pacotes instalados

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
