# apptainer

Gestisce container per HPC e calcolo scientifico. Alcuni sottocomandi come `build`, `pull` e `push` hanno la propria documentazione d'uso.

## Usage
- `apptainer pull path/to/immagine.sif docker://immagine:tag` : scarica un container da Docker Hub
- `apptainer pull path/to/immagine.sif library://utente/collezione/container:tag` : scarica un container dalla Container Library
- `apptainer build path/to/immagine.sif path/to/definizione.def` : costruisce un container da un file di definizione
- `apptainer shell path/to/immagine.sif` : avvia una shell interattiva all'interno di un container
- `apptainer exec path/to/immagine.sif comando` : esegue un comando all'interno di un container
- `apptainer run path/to/immagine.sif` : esegue lo script di avvio predefinito di un container
- `apptainer inspect path/to/immagine.sif` : ispeziona i metadati di un container
- `apptainer -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli.html
