# apt-mark

Utilitário que altera as configurações dos pacotes instalados.

## Usage
- `sudo apt-mark auto nome_do_pacote` : marca um pacote como instalado automaticamente
- `sudo apt-mark hold nome_do_pacote` : bloqueia um pacote na sua versão atual, impedindo que ele seja atualizado
- `sudo apt-mark unhold nome_do_pacote` : desbloqueia um pacote, permitindo que ele seja atualizado
- `apt-mark showmanual` : lista os pacotes instalados manualmente
- `apt-mark showhold` : lista os pacotes bloqueados

## Links
- More information: https://manned.org/apt-mark
