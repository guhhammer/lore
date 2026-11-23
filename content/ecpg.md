# ecpg

Embedded SQL preprocessor for C programs.

## Usage
- `ecpg input.pgc` : preprocess a specific file
- `ecpg -o - -` : preprocess from `stdin` and output to `stdout`
- `cat input.pgc | ecpg -o output.c -` : preprocess from `stdin` and write to a file
- `ecpg -o output.c input.pgc` : preprocess and specify an output file
- `ecpg input.pgh` : preprocess a header file (`.pgh` extension)
- `ecpg -C INFORMIX|INFORMIX_SE|ORACLE input.pgc` : preprocess in a specific compatibility mode
- `ecpg -t input.pgc` : preprocess with autocommit enabled

## Categories
files, databases

## Links
- More information: https://www.postgresql.org/docs/current/app-ecpg.html
- tldr page: https://tldr.inbrowser.app/pages/common/ecpg
