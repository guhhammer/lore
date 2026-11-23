# as

Assembler GNU portabile. Progettato principalmente per assemblare l'output di `gcc` ed utilizzarlo con `ld`.

## Usage
- `as percorso/del/file.s` : assembla un file, scrivendo l'output su a.out
- `as percorso/del/file.s -o percorso/del/out.o` : assembla l'output nel file dato
- `as -f percorso/del/file.s` : genera l'output più velocemente saltando gli spazi e senza preprocessare i commenti. (Questo comando dovrebbe essere utilizzato solo con compilatori fidati)
- `as -I percorso/della/directory percorso/del/file.s` : includi un percorso dato alla lista delle directory in cui cercare i file specificati nelle direttive `.include`

## Links
- More information: https://keith.github.io/xcode-man-pages/as.1.html
