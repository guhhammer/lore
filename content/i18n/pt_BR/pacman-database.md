# pacman-database

Atua no banco de dados de pacotes do Arch Linux. Modifica certos atributos dos pacotes instalados.

## Usage
- `sudo pacman -D --asdeps pacote` : marca um pacote como instalado implicitamente
- `sudo pacman -D --asexplicit pacote` : marca um pacote como instalado explicitamente
- `pacman -Dk` : verifica se todas as dependências de pacotes estão instaladas
- `pacman -Dkk` : verifica os repositórios para garantir que todas as dependências especificadas estejam disponíveis
- `pacman -Dkq` : exibe apenas mensagens de erro
- `pacman -Dh` : exibe ajuda

## Links
- More information: https://manned.org/pacman.8
