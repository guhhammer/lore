# pacaur

Um utilitário para Arch Linux para construir e instalar pacotes a partir do Arch User Repository.

## Usage
- `pacaur -Syu` : sincroniza e atualiza todos os pacotes (inclui o AUR)
- `pacaur -Syua` : sincroniza e atualiza apenas os pacotes do AUR
- `pacaur -S pacote` : instala um novo pacote (inclui o AUR)
- `pacaur -Rs pacote` : remove um pacote e suas dependências (inclui pacotes do AUR)
- `pacaur -Ss palavra_chave` : pesquisa o banco de dados de pacotes para uma palavra-chave (inclui o AUR)
- `pacaur -Qs` : lista todos os pacotes atualmente instalados (inclui pacotes do AUR)

## Links
- More information: https://github.com/rmarquis/pacaur#name
