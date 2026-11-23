# clamscan

Scanner antivirus da linea di comando.

## Usage
- `clamscan percorso/del/file` : analizza un file cercando vulnerabilità
- `clamscan -r percorso/della/directory` : analizza ricorsivamente tutti i file in una specifica directory
- `comando | clamscan -` : analizza dati da `stdin`
- `clamscan -d percorso/del/file_o_directory` : specifica un file o directory di file da usare come database virus
- `clamscan -i` : analizza la directory corrente e mostra in output solo i file infetti
- `clamscan -l percorso/del/file_log` : scrivi il risultato di uno scan in un file di log
- `clamscan --move percorso/della/directory_quarantena` : sposta i file infetti in una specifica directory
- `clamscan --remove yes` : elimina i file infetti

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamscan
