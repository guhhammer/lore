# md5sum

Calcola i checksum crittografici di tipo MD5.

## Usage
- `md5sum percorso/del/file` : calcolare il checksum MD5 di un file
- `md5sum percorso/del/file1 percorso/del/file2` : calcola i checksum MD5 per più di un file
- `md5sum -c percorso/del/file.md5` : verifica che tutti i file abbiano checksum corrispondenti al file di MD5SUM
- `echo "testo" | md5sum` : calcola il checksum MD5 da `stdin`
