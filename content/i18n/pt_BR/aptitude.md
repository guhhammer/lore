# aptitude

Utilitário de gerenciamento de pacotes de Debian e Ubuntu.

## Usage
- `sudo aptitude update` : sincroniza a lista de pacotes e versões disponíveis. Deve ser executado antes de outros comandos `aptitude`
- `sudo aptitude install pacote` : instala um novo pacote e suas dependências
- `aptitude search pacote` : busca por um determinado pacote
- `aptitude search '?installed(pacote)'` : busca por uma determinado pacote instalado (`?installed` é um termo de busca `aptitude`)
- `sudo aptitude remove pacote` : remove um pacote e todos que dependam dele
- `sudo aptitude upgrade` : atualiza os pacotes instalados para suas versões mais recentes
- `sudo aptitude full-upgrade` : atualiza os pacotes instalados (semelhante ao `upgrade`), porém removendo os obsoletos e instalando pacotes solicitados por novas dependências
- `sudo aptitude hold '?installed(pacote)'` : coloca um pacote instalado em espera para prevenir atualizações automáticas

## Links
- More information: https://manned.org/aptitude
