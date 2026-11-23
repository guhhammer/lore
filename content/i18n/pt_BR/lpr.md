# lpr

Ferramenta do CUPS para imprimir arquivos.

## Usage
- `lpr caminho/para/arquivo` : imprime um arquivo na impressora padrão
- `lpr -# 2 caminho/para/arquivo` : imprime 2 cópias
- `lpr -P impressora caminho/para/arquivo` : imprime em uma impressora específica
- `lpr -o page-ranges=2|2-16 caminho/para/arquivo` : imprime uma única página (p. ex., 2) ou uma faixa de páginas (p. ex., 2-16)
- `lpr -o sides=two-sided-long-edge|two-sided-short-edge caminho/para/arquivo` : imprime frente e verso em modo retrato (long) ou paisagem (short)
- `lpr -o media=a4|letter|legal caminho/para/arquivo` : define o tamanho da página (mais opções podem estar disponíveis dependendo da configuração)
- `lpr -o number-up=2|4|6|9|16 caminho/para/arquivo` : imprime múltiplas páginas por folha

## Links
- More information: https://openprinting.github.io/cups/doc/man-lpr.html
