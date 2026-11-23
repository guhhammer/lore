# brctl

Administração de pontes de rede.

## Usage
- `sudo brctl show` : exibe uma lista com informações das pontes de rede existentes
- `sudo brctl add nome_da_ponte` : cria uma ponte de rede
- `sudo brctl del nome_da_ponte` : remove uma ponte de rede
- `sudo brctl addif nome_da_ponte nome_da_interface_de_rede` : adiciona uma interface de rede em uma ponte de rede existente
- `sudo brctl delif nome_da_ponte nome_da_interface_de_rede` : remove uma interface de rede de uma ponte de rede existente

## Links
- More information: https://manned.org/brctl
