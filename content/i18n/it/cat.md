# cat

Stampa e concatena file.

## Usage
- `cat percorso/del/file` : stampa il contenuto di un file su `stdout`
- `cat percorso/del/file1 percorso/del/file2 ... >percorso/del/file_di_output` : concatena diversi file in un file di output
- `cat percorso/del/file1 percorso/del/file2 ... >>percorso/del/file_di_output` : aggiunge diversi file a un file di output
- `cat -u /dev/tty12 >/dev/tty13` : copia il contenuto di un file in un file di output senza buffering
- `cat - >percorso/del/file` : scrive `stdin` in un file

## Links
- More information: https://manned.org/cat.1posix
