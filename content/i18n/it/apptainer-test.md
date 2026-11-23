# apptainer test

Esegue lo script di test definito in un'immagine container Apptainer. Lo script di test è definito nella sezione `%test` del file di definizione del container.

## Usage
- `apptainer test path/to/immagine.sif` : esegue lo script di test di un container
- `apptainer test -B path/to/source:path/to/destinazione path/to/immagine.sif` : esegue lo script di test con un mount bind dall'host al container
- `apptainer test -C path/to/immagine.sif` : esegue lo script di test in modalità completamente isolata (filesystem, PID, IPC e ambiente pulito)
- `apptainer test --writable-tmpfs path/to/immagine.sif` : esegue lo script di test con un overlay del filesystem temporaneo scrivibile
- `apptainer test --app nome_app path/to/immagine.sif` : esegue lo script di test per un'app specifica all'interno di un container
- `apptainer test -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_test.html
