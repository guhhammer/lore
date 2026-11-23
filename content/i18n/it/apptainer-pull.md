# apptainer pull

Scarica immagini container da sorgenti remote.

## Usage
- `apptainer pull path/to/immagine.sif docker://immagine:tag` : scarica un container da Docker Hub
- `apptainer pull path/to/immagine.sif library://utente/collezione/container:tag` : scarica un container dalla Container Library
- `apptainer pull path/to/immagine.sif oras://registry/namespace/immagine:tag` : scarica un container da un registro OCI
- `apptainer pull --arch amd64|arm64|ppc64le path/to/immagine.sif library://immagine:tag` : scarica un container per un'architettura specifica
- `apptainer pull -F path/to/immagine.sif docker://immagine:tag` : forza la sovrascrittura di un file immagine esistente
- `apptainer pull --sandbox percorso/della/directory docker://immagine:tag` : scarica un container come directory sandbox scrivibile
- `apptainer pull --disable-cache path/to/immagine.sif docker://immagine:tag` : scarica un container senza utilizzare la cache
- `apptainer pull -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_pull.html
