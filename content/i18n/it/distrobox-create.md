# distrobox-create

Crea un container Distrobox. Il container creato sarà strettamente integrato con l'host, consentendo la condivisione della directory `$HOME` dell'utente, storage esterno, dispositivi USB esterni, app grafiche (X11/Wayland) e audio.

## Usage
- `distrobox-create container_name -i ubuntu:latest` : crea un container Distrobox usando l'immagine Ubuntu
- `distrobox-create -c container_name cloned_container_name` : clona un container Distrobox

## Links
- More information: https://distrobox.it/usage/distrobox-create/
