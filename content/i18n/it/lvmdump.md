# lvmdump

Raccoglie informazioni diagnostiche su LVM2 (Logical Volume Manager). Per impostazione predefinita, genera un archivio `.tar` compresso con dati di sistema e configurazione nella directory home.

## Usage
- `lvmdump` : genera un dump base
- `lvmdump -a -l -m` : genera un dump esteso con metadati e informazioni sui daemon
- `lvmdump -d percorso/della/directory` : salva le informazioni in una directory invece che in un tarball
- `lvmdump -h` : mostra l'aiuto

## Links
- More information: https://manned.org/lvmdump
