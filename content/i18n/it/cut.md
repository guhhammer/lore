# cut

Taglia dividendo in campi `stdin` o file.

## Usage
- `cut -c 1-16` : estrai i primi 16 caratteri di ogni riga da `stdin`
- `cut -c 1-16 file` : estrai i primi 16 caratteri di ogni riga da un dato file
- `cut -c 3-` : estrai tutto dal terzo carattere fino alla fine di ogni riga
- `cut -d':' -f5` : estrai il quinto campo di ogni linea, utilizzando i due punti come separatore di campo (il default è tab)
- `cut -d';' -f2,10` : estrai il secondo e decimo campo di ogni linea, utilizzando il punto e virgola come delimitatore
- `cut -d' ' -f3-` : estrai i campi dal terzo in poi di ogni linea, utilizzando lo spazio come delimitatore

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cut-invocation.html
