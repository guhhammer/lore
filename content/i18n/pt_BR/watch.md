# watch

Execute um programa periodicamente e monitore a saída no modo de tela cheia.

## Usage
- `watch comando` : executar repetidamente um comando e mostrar o resultado
- `watch -n 60 comando` : executar novamente um comando a cada 60 segundos
- `watch -d df` : monitore o espaço em disco, destacando as diferenças à medida que elas aparecem
- `watch "comando_1 | comando_2 | comando_3"` : executar repetidamente um pipeline e mostrar o resultado
- `watch -g lsblk` : saia do `watch` se a saída visível for alterada
- `watch -c ls --color=always` : interpretar caracteres de controle do terminal

## Links
- More information: https://manned.org/watch
