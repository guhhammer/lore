# flatpak

Constrói, instala e executa aplicações e plataformas flatpak.

## Usage
- `flatpak run com.exemplo.aplicacao` : executa uma aplicação instalada
- `flatpak install nome_remoto com.exemplo.aplicacao` : instala uma aplicação de uma fonte remota
- `flatpak list --app` : lista aplicações instaladas, ignorando plataformas
- `flatpak update` : atualiza todas as aplicações e plataformas instaladas
- `flatpak remote-add --if-not-exists nome_remoto url_remoto` : adiciona uma fonte remota
- `flatpak remove com.exemplo.aplicacao` : remove uma aplicação instalada
- `flatpak remove --unused` : remove todos as aplicações não usadas
- `flatpak info com.exemplo.aplicacao` : mostra informações sobre uma aplicação instalada

## Links
- More information: https://docs.flatpak.org/en/latest/flatpak-command-reference.html#flatpak
