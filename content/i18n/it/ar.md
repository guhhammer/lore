# ar

Crea, modifica ed estrai da archivi (`.a`, `.so`, `.o`).

## Usage
- `ar x percorso/del/file.a` : estrai (x) tutti i membri da un archivio
- `ar t percorso/del/file.ar` : lista tutti i membri di un archivio
- `ar r percorso/del/file.deb percorso/del/debian-binary percorso/del/control.tar.gz percorso/del/data.tar.xz ...` : sostituisci (r) o aggiungi file ad un archvio
- `ar s percorso/del/file.a` : inserisci o sostituisci un indice in un archivio (equivalente ad usare `ranlib`)
- `ar rs percorso/del/file.a percorso/del/file1.o percorso/del/file2.o ...` : crea un archivio con dei file creando anche il relativo indice

## Links
- More information: https://manned.org/ar
