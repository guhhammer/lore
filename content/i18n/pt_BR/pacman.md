# pacman

Utilitário de Arch Linux para gerenciamento de pacotes.

## Usage
- `sudo pacman -Syu` : sincroniza e atualiza todos os pacotes
- `sudo pacman -S pacote` : instala um novo pacote
- `sudo pacman -Rs pacote` : remove um pacote e suas dependências
- `pacman -F "nome_do_arquivo"` : procura pacotes no banco de dados que contenham um arquivo específico
- `pacman -Q` : lista pacotes instalados e versões
- `pacman -Qe` : lista apenas os pacotes explicitamente instalados e versões
- `pacman -Qtdq` : lista pacotes órfãos (instalado como dependência mas não requerido por qualquer pacote)
- `sudo pacman -Scc` : esvazia completamente o cache do pacman

## Links
- More information: https://manned.org/pacman.8
