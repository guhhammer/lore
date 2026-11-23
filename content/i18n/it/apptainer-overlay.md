# apptainer overlay

Gestisce immagini overlay EXT3 scrivibili per container Apptainer.

## Usage
- `apptainer overlay create -s dimensione path/to/immagine.sif` : aggiunge un overlay scrivibile a un'immagine SIF esistente
- `apptainer overlay create -s dimensione path/to/overlay.img` : crea un'immagine overlay EXT3 scrivibile autonoma
- `apptainer overlay create -s dimensione -S path/to/overlay.img` : crea un'immagine overlay sparse
- `apptainer overlay create -f -s dimensione path/to/overlay.img` : crea un overlay per l'uso con fakeroot
- `apptainer overlay create --create-dir percorso/della/directory path/to/overlay.img` : crea un overlay con una directory specifica nel layout
- `apptainer overlay -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_overlay.html
