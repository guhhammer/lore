# apptainer push

Invia immagini container a registri remoti.

## Usage
- `apptainer push path/to/immagine.sif library://utente/collezione/container:tag` : invia un container alla Container Library
- `apptainer push path/to/immagine.sif oras://registry/namespace/immagine:tag` : invia un container a un registro OCI
- `apptainer push -U path/to/immagine.sif library://utente/collezione/container:tag` : invia un container non firmato (salta la verifica della firma)
- `apptainer push -D "descrizione" path/to/immagine.sif library://utente/collezione/container:tag` : invia un container con una descrizione (solo library)
- `apptainer push -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_push.html
