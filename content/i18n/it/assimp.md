# assimp

Client da linea di comando per la Open Asset Import Library. Supporta il caricamento di 40+ formati di file per modelli 3D, e l'espoerazione di diversi formati 3D popolari.

## Usage
- `assimp listext` : elenca tutti i formati supportati
- `assimp listexport` : elenca tutti i formati supportati per l'esportazione
- `assimp export file_input.stl file_output.obj` : converti un file a uno dei tipi supportati, utilizzando i parametri predefiniti
- `assimp export file_input.stl file_output.obj parametri` : converti un file utilizzando parametri personalizzati (il file `dox_cmd.h` nel source code di assimp contiene una lista di parametri disponibili)
- `assimp info percorso/del/file` : mostra un riepilogo del contenuto di un file
- `assimp help` : mostra l'aiuto
- `assimp sottocomando --help` : chiedi aiuto per uno specifico sottocomando (ad esempio riguardo i suoi parametri)

## Links
- More information: https://manned.org/assimp
