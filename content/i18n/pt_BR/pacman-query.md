# pacman-query

Utilitário de Arch Linux para gerenciamento de pacotes.

## Usage
- `pacman -Q` : lista pacotes instalados e suas versões
- `pacman -Qe` : lista apenas pacotes e versões que foram instalados explicitamente
- `pacman -Qo arquivo` : procura qual pacote possui um arquivo
- `pacman -Qi pacote` : exibe informações sobre um pacote instalado
- `pacman -Ql pacote` : lista arquivos fornecidos por um pacote
- `pacman -Qdtq` : lista pacotes órfãos (instalados como dependências, mas que nenhum pacote instalado necessita)
- `pacman -Qm` : lista pacotes instalados não encontrados nos repositórios
- `pacman -Qu` : lista pacotes desatualizados

## Links
- More information: https://manned.org/pacman.8
