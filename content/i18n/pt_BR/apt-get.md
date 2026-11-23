# apt-get

Gerenciador de pacotes das distribuições baseadas em Debian. Procure por pacotes utilizando o `apt-cache`.

## Usage
- `sudo apt-get update` : atualiza a lista de pacotes disponíveis (recomenda-se executá-lo antes de outros comandos `apt-get`)
- `sudo apt-get install nome_do_pacote` : instala um pacote ou atualizá-lo para a versão mais recente
- `sudo apt-get remove nome_do_pacote` : remove um pacote
- `sudo apt-get purge nome_do_pacote` : remove um pacote e os seus arquivos de configuração
- `sudo apt-get upgrade` : atualiza todos os pacotes instalados para as versões mais recentes
- `sudo apt-get autoclean` : limpa o repositório local — removendo os arquivos de pacotes (`.deb`) de downloads interrompidos que não podem mais ser baixados
- `sudo apt-get autoremove` : remove todos os pacotes obsoletos
- `sudo apt-get dist-upgrade` : atualiza os pacotes instalados (semelhante ao `upgrade`), porém removendo os obsoletos e instalando pacotes solicitados por novas dependências

## Links
- More information: https://manned.org/apt-get.8
