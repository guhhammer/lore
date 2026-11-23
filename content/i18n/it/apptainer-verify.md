# apptainer verify

Verifica le firme digitali delle immagini container SIF.

## Usage
- `apptainer verify path/to/immagine.sif` : verifica un'immagine container utilizzando il portachiavi PGP predefinito
- `apptainer verify --key path/to/pubblica.pem path/to/immagine.sif` : verifica un'immagine container utilizzando un file di chiave pubblica specifico
- `apptainer verify --certificate path/to/certificato.pem path/to/immagine.sif` : verifica un'immagine container utilizzando un file certificato
- `apptainer verify -a path/to/immagine.sif` : verifica tutti gli oggetti nell'immagine
- `apptainer verify -g id_gruppo path/to/immagine.sif` : verifica un gruppo di oggetti specifico all'interno dell'immagine
- `apptainer verify -i id_oggetto path/to/immagine.sif` : verifica un oggetto specifico per ID all'interno dell'immagine
- `apptainer verify -j path/to/immagine.sif` : mostra i risultati della verifica in formato JSON
- `apptainer verify -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_verify.html
