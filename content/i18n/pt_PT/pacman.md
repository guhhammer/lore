# pacman

Utilitário para gerir pacotes Arch Linux.

## Usage
- `sudo pacman -Syu` : sincroniza e actualiza todos os pacotes
- `sudo pacman -S package_name` : instala um novo pacote
- `sudo pacman -Rs nome_do_pacote` : remove um pacote e todas as dependencias
- `pacman -Q` : lista versão dos pactotes instalados
- `pacman -Qe` : lista versão dos pactotes instalados explicitamente
- `pacman -Qtdq` : lista pacotes órfãos (instalados como dependencia mas não exigidos por nenhum pacote)
- `sudo pacman -Scc` : remove memória armazenada (cache) do `pacman`

## Links
- More information: https://manned.org/pacman.8
