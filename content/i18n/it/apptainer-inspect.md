# apptainer inspect

Mostra i metadati delle immagini container Apptainer.

## Usage
- `apptainer inspect path/to/immagine.sif` : mostra le etichette di un'immagine (predefinito)
- `apptainer inspect -d path/to/immagine.sif` : mostra il file di definizione utilizzato per costruire l'immagine
- `apptainer inspect -r path/to/immagine.sif` : mostra lo script di avvio per l'immagine
- `apptainer inspect -e path/to/immagine.sif` : mostra le variabili d'ambiente dell'immagine
- `apptainer inspect --list-apps path/to/immagine.sif` : elenca tutte le app nel container
- `apptainer inspect --all path/to/immagine.sif` : mostra tutti i dati disponibili in formato JSON
- `apptainer inspect -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_inspect.html
