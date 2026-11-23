# emerge

Utilitário de gerenciamento de pacotes do Gentoo Linux.

## Usage
- `sudo emerge --sync` : sincronizar todos os pacotes
- `sudo emerge -avuDN @world` : atualiza todos os pacotes, incluindo dependências
- `sudo emerge --resume --skipfirst` : resume uma atualização falha, pulando o pacote que falhou
- `sudo emerge -av pacote` : instala um novo pacote, com confirmação
- `sudo emerge -avc pacote` : remove um pacote e suas dependências com confirmação
- `sudo emerge -avc` : remove pacotes órfãos (instalados como dependências mas não são mais requisitados por nenhum pacote)
- `emerge -S palavra_chave` : procura na base de dados por uma palavra-chave

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
