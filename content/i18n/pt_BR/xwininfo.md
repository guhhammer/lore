# xwininfo

Mostra informações sobre janelas.

## Usage
- `xwininfo` : mostra um cursor para selecionar uma janela para mostrar seus atributos (ID, nome, tamanho, posição...)
- `xwininfo -tree -root` : mostra a árvore de todas as janelas
- `xwininfo -id id` : mostra os atributos de uma janela com um ID específico
- `xwininfo -name nome` : mostra os atributos de uma janela com um nome específico
- `xwininfo -tree -root | grep palavra_chave | head -1 | perl -ne 'print $1 if /(0x[\da-f]+)/ig;'` : mostra o ID de uma janela buscando pelo nome

## Links
- More information: https://www.x.org/releases/current/doc/man/man1/xwininfo.1.xhtml
