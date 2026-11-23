# yaourt

Utilitário de Arch Linux para compilaçào de pacotes AUR (Arch User Repository).

## Usage
- `yaourt -Syua` : sincroniza e atualiza todos os pacotes (incluindo AUR)
- `yaourt -S nome_do_pacote` : instala um novo pacote (incluindo AUR)
- `yaourt -Rs nome_do_pacote` : remove um pacote e suas dependências (incluindo pacotes AUR)
- `yaourt -Ss nome_do_pacote` : procura no banco de dados de pacotes por uma palavra-chave (incluindo AUR)
- `yaourt -Q` : lista pacotes instalados, versões, e repositórios (pacotes AUR serão listados sob como repositório 'local')

## Links
- More information: https://archlinux.fr/yaourt-en
