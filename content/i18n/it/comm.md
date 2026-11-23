# comm

Seleziona o ignora linee comuni a due file. Entrambi i file devono essere ordinati.

## Usage
- `comm file1 file2` : produci tre colonne separate da tab: linee solo nel primo file, linee solo nel secondo file, e linee comuni ad entrambi
- `comm -12 file1 file2` : stampa solo le linee comune ad entrambi i file
- `cat file1 | comm -12 - file2` : stampa solo le lin comuni ad entrambi i file, leggendone uno da `stdin`
- `comm -23 file1 file2 >file3` : filtra le linee trovate solo nel primo file, salvando il risultato in un terzo file
- `comm -13 <(sort file1) <(sort file2)` : filtra le linee trovate solo nel secondo file, con due file che non sono ordinati

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/comm-invocation.html
