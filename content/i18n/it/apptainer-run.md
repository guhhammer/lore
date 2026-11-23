# apptainer run

Esegue lo script di avvio predefinito di un container Apptainer. Lo script di avvio è definito nella sezione `%runscript` del file di definizione del container.

## Usage
- `apptainer run path/to/immagine.sif` : esegue lo script di avvio predefinito di un container
- `apptainer run path/to/immagine.sif arg1 arg2 ...` : esegue con argomenti passati allo script di avvio
- `apptainer run -B path/to/source:path/to/destinazione path/to/immagine.sif` : esegue con un mount bind dall'host al container
- `apptainer run --env variabile=valore path/to/immagine.sif` : esegue con variabili d'ambiente
- `apptainer run -C path/to/immagine.sif` : esegue in modalità completamente isolata (filesystem, PID, IPC e ambiente pulito)
- `apptainer run --writable-tmpfs path/to/immagine.sif` : esegue con un overlay del filesystem temporaneo scrivibile
- `apptainer run --nv path/to/immagine.sif` : esegue con supporto GPU NVIDIA
- `apptainer run -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_run.html
