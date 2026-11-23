# apptainer build

Costruisce immagini container Apptainer.

## Usage
- `apptainer build path/to/immagine.sif path/to/definizione.def` : costruisce un container da un file di definizione
- `apptainer build path/to/immagine.sif docker://immagine:tag` : costruisce un container da Docker Hub
- `apptainer build path/to/immagine.sif library://utente/collezione/container:tag` : costruisce un container dalla Container Library
- `apptainer build -s percorso/della/directory docker://immagine:tag` : costruisce una directory sandbox scrivibile invece di un file immagine
- `apptainer build --disable-cache path/to/immagine.sif docker://immagine:tag` : costruisce un container senza utilizzare la cache
- `apptainer build -F path/to/immagine.sif path/to/definizione.def` : forza la sovrascrittura di un file immagine esistente
- `apptainer build -f path/to/immagine.sif path/to/definizione.def` : costruisce utilizzando fakeroot per build non privilegiate
- `apptainer build -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_build.html
