# chkdsk

Controlla il file system e i metadata dei dischi per cercare errori.

## Usage
- `chkdsk disco` : specifica la lettera del disco (seguita da due punti ':'), monta una partizione o un disco da controllare
- `chkdsk disco /f` : ripara gli errori di un disco specifico
- `chkdsk volume /x` : smonta un disco specifico prima di eseguire il controllo
- `chkdsk /ldimensione` : cambia la dimensione dei file di log in una dimensione specifica (solo per NTFS)
