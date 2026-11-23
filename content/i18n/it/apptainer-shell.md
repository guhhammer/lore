# apptainer shell

Avvia una shell interattiva all'interno di un container Apptainer.

## Usage
- `apptainer shell path/to/immagine.sif` : avvia una shell interattiva all'interno di un container
- `apptainer shell -B path/to/source:path/to/destinazione path/to/immagine.sif` : avvia una shell con un mount bind dall'host al container
- `apptainer shell --env variabile=valore path/to/immagine.sif` : avvia una shell con variabili d'ambiente
- `apptainer shell -C path/to/immagine.sif` : avvia una shell in modalità completamente isolata (filesystem, PID, IPC e ambiente pulito)
- `apptainer shell --writable-tmpfs path/to/immagine.sif` : avvia una shell con un overlay del filesystem temporaneo scrivibile
- `apptainer shell --nv path/to/immagine.sif` : avvia una shell con supporto GPU NVIDIA
- `apptainer shell -s path/to/shell path/to/immagine.sif` : avvia una shell utilizzando un programma shell specifico
- `apptainer shell -h` : mostra l'aiuto

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_shell.html
