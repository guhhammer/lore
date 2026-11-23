# apptainer exec

Esegue un comando all'interno di un container Apptainer.

## Usage
- `apptainer exec path/to/immagine.sif comando` : esegue un comando all'interno di un container
- `apptainer exec path/to/immagine.sif comando arg1 arg2 ...` : esegue un comando con argomenti
- `apptainer exec -B path/to/source:path/to/destinazione path/to/immagine.sif comando` : esegue un comando con un mount bind dall'host al container
- `apptainer exec --env variabile=valore path/to/immagine.sif comando` : esegue un comando con variabili d'ambiente
- `apptainer exec -C path/to/immagine.sif comando` : esegue un comando in modalità completamente isolata (filesystem, PID, IPC e ambiente pulito)
- `apptainer exec --writable-tmpfs path/to/immagine.sif comando` : esegue un comando con un overlay del filesystem temporaneo scrivibile
- `apptainer exec --nv path/to/immagine.sif comando` : esegue un comando con supporto GPU NVIDIA
- `apptainer exec -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_exec.html
