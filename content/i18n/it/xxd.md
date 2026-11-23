# xxd

Mostra la rappresentazione esadecimale (hexdump) di un file binario e viceversa.

## Usage
- `xxd file_di_input` : creare l'hexdump di un file binario e mostrare l'output
- `xxd file_di_input file_di_output` : creare l'hexdump di un file binario e salvare il risultato in un file
- `xxd -a file_di_input` : mostrare un output in una versione un po' più compatta, dove gli zero consegutivi vengono sostituiti da un asterisco
- `xxd -c 10 file_di_input` : mostrare l'output in 10 colonne di un ottetto (byte) ciascuna
- `xxd -l 32 file_di_input` : mostrare l'output solo fino ad una lunghezza massima di 32 bytes
- `xxd -p file_di_input` : mostrare l'output in modalità normale, senza spazi tra le colonne
- `xxd -r -p file_di_input file_di_output` : eseguire l'operazione inversa, ovvero da un hexdump creare il binario e salvarlo in un file

## Links
- More information: https://manned.org/xxd
