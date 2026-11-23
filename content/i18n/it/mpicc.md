# mpicc

Involucro Open MPI per il compilatore di C. Shell che esegue sul compilatore, aggiungono i relevanti argomenti e linkers necessari a compilare/collegare programmi Open MPI, invocando il sottostante compilatore di C per effettuare le effetive operazioni.

## Usage
- `mpicc -c percorso/del/file.c` : compila un file sorgente in un file oggetto
- `mpicc -o executable percorso/del/file.o` : linka un file oggetto file e genera un eseguibile
- `mpicc -o executable percorso/del/file.c` : linka e compila i file sorgente in un solo commando

## Links
- More information: https://www.mpich.org/static/docs/latest/www1/mpicc.html
