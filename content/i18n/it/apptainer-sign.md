# apptainer sign

Aggiunge firme digitali a un'immagine container SIF.

## Usage
- `apptainer sign path/to/immagine.sif` : firma un'immagine container utilizzando la chiave PGP predefinita
- `apptainer sign --key path/to/privata.pem path/to/immagine.sif` : firma un'immagine container utilizzando un file di chiave privata specifico
- `apptainer sign -k indice_chiave path/to/immagine.sif` : firma un'immagine container utilizzando un indice di chiave PGP specifico
- `apptainer sign -g id_gruppo path/to/immagine.sif` : firma un gruppo di oggetti specifico all'interno dell'immagine
- `apptainer sign -i id_oggetto path/to/immagine.sif` : firma un oggetto specifico per ID all'interno dell'immagine
- `apptainer sign -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_sign.html
