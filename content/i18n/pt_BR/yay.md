# yay

Yet Another Yogurt: Um utilitário de Arch Linux para compilar e instalar pacotes do AUR (Arch User Repository).

## Usage
- `yay nome_do_pacote|termo_de_busca` : busca interativamente e instala pacotes dos repositórios e AUR
- `yay` : sincroniza e atualiza todos os pacotes dos repositórios e AUR
- `yay -Sua` : sincroniza e atualiza apenas pacotes AUR
- `yay -S nome_do_pacote` : instala um novo pacote de repositório e AUR
- `yay -Rns nome_do_pacote` : remove um pacote instalado, suas dependências e arquivos de configuração
- `yay -Ss palavra_chave` : procura no banco de dados de pacotes por uma palavra-chave dos repositórios e AUR
- `yay -Yc` : remove pacotes órfãos (instalado como dependência mas não utilizado por qualquer pacote)
- `yay -Ps` : mostra estatísticas dos pacotes instalados e condição do sistema

## Links
- More information: https://github.com/Jguer/yay#first-use
