# envsubst

Sostituisci variabili di ambiente con il loro valore in stringhe di formato della shell. Le variabili da sostituire devono essere nella forma `${var}` oppure `$var`.

## Usage
- `echo '$HOME' | envsubst` : sostituisci variabili di ambiente in `stdin` e stampa l'output su `stdout`
- `envsubst <percorso/del/file_input` : sostituisci variabili di ambiente in un file input e stampa l'output su `stdout`
- `envsubst <percorso/del/file_input >percorso/del/file_output` : sostituisci variabili di ambiente in un file input e scrivi l'output su un file
- `envsubst <percorso/del/file_input '$USER $HOME $SHELL'` : sostituisci in un file input le variabili di ambiente specificate in una lista separata da spazi

## Links
- More information: https://www.gnu.org/software/gettext/manual/gettext.html#envsubst-Invocation
