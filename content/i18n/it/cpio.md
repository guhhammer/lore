# cpio

Copia file da/a archivi. Supporta i seguenti formati di archivio: cpio binario, vecchio ASCII, nuovo ASCII, crc, HPUX binario, HPUX vecchio ASCII, vecchio tar, e tar POSIX.1.

## Usage
- `echo "percorso/del/file1 percorso/del/file2 ..." | cpio -o >archivio.cpio` : accetta una lista di nomi di file da `stdin` ed aggiungili ad un archivio (copy-out) in formato binario cpio
- `find percorso/della/directory | cpio -ov >archivio.cpio` : copia tutti i file e le directory in una directory ed aggiungili ad un archivio (copy-out), in modalità verbosa
- `cpio <archivio.cpio -idv` : estrai file da un archivio (copy-in), generando le directory necessarie, in modalità verbosa

## Links
- More information: https://www.gnu.org/software/cpio/manual/cpio.html#Invoking-cpio
