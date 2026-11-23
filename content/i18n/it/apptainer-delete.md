# apptainer delete

Elimina immagini container da un repository remoto.

## Usage
- `apptainer delete library://utente/collezione/container:tag` : elimina un'immagine dalla Container Library
- `apptainer delete -A amd64|arm64|ppc64le library://utente/collezione/container:tag` : elimina un'immagine per un'architettura specifica
- `apptainer delete -F library://utente/collezione/container:tag` : forza l'eliminazione di un'immagine senza conferma
- `apptainer delete --library https://library.example.com library://utente/collezione/container:tag` : elimina un'immagine da un server library specifico
- `apptainer delete --no-https library://hostname/utente/collezione/container:tag` : elimina un'immagine utilizzando HTTP invece di HTTPS
- `apptainer delete -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_delete.html
