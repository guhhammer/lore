# bash

Bourne-Again SHell, interprete di comandi da linea di comando compatibile con `sh`.

## Usage
- `bash` : avvia una sessione shell interattiva
- `bash --norc` : avvia una sessione shell interattiva senza caricare i file di configurazione di avvio
- `bash -c "echo 'bash è eseguito'"` : esegue comandi specifici
- `bash percorso/allo/script.sh` : esegue uno script specifico
- `bash -x percorso/allo/script.sh` : esegue uno script specifico stampando ogni comando prima dell'esecuzione
- `bash -e percorso/allo/script.sh` : esegue uno script specifico e si ferma al primo errore
- `echo "echo 'bash è eseguito'" | bash` : esegue comandi specifici da `stdin`
- `bash -r` : avvia una sessione shell ristretta

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Invoking-Bash
