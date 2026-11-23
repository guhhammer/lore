# b2sum

Calcola checksum crittografici BLAKE2.

## Usage
- `b2sum percorso/del/file1 percorso/del/file2 ...` : calcola il checksum BLAKE2 per uno o più file
- `b2sum percorso/del/file1 percorso/del/file2 ... >percorso/del/file.b2` : calcola e salva l'elenco dei checksum BLAKE2 in un file
- `comando | b2sum` : calcola un checksum BLAKE2 da `stdin`
- `b2sum -c percorso/del/file.b2` : legge un file di checksum BLAKE2 e nomi di file e verifica che tutti i file abbiano checksum corrispondenti
- `b2sum -c --quiet percorso/del/file.b2` : mostra un messaggio solo per file mancanti o quando la verifica fallisce
- `b2sum --ignore-missing -c --quiet percorso/del/file.b2` : mostra un messaggio solo quando la verifica fallisce, ignorando i file mancanti
- `echo checksum_blake2_conosciuto_del_file percorso/del/file | b2sum -c` : verifica un checksum BLAKE2 noto di un file
