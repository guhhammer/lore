# bspc

Configura e controla `bspwm`, gerenciando nós, áreas de trabalho, monitores, e outros.

## Usage
- `bspc monitor -d nome_da_area_1 nome_da_area_2` : define duas áreas de trabalho virtuais
- `bspc desktop -f numero` : foca em uma área de trabalho determinada
- `bspc node -c` : fecha as janelas atreladas ao nó selecionado
- `bspc node -d numero` : envia o nó selecionado para uma área de trabalho determinada
- `bspc node -t ~fullscreen` : alterna o nó selecionado para modo de tela cheia
- `bspc config nome_da_configuracao valor` : define o valor de uma configuração específica

## Links
- More information: https://github.com/baskerville/bspwm/blob/master/doc/bspwm.1.asciidoc
