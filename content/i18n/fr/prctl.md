# prctl

Obtient ou définit les contrôles de ressources des processus, tâches et projets en cours d'exécution.

## Usage
- `prctl id_processus` : examine les limites et les autorisations des processus
- `prctl -P id_processus` : examine les limites et les autorisations de processus dans un format analysable par machine
- `prctl -n process.max-file-descriptor id_processus` : obtient une limite spécifique pour un processus en cours d'exécution

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
