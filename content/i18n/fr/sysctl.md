# sysctl

Liste et modifie les variables d'exécution du noyau.

## Usage
- `sysctl -a` : affiche toutes les variables disponibles et leurs valeurs
- `sysctl -w section.modifiable=valeur` : définis une variable d'état modifiable du noyau
- `sysctl fs.file-nr` : obtiens les gestionnaires de fichiers (handlers) actuellement ouverts
- `sysctl fs.file-max` : obtiens la limite de nombre de fichiers ouverts simultanément
- `sysctl -p` : applique les changements de `/etc/sysctl.conf`

## Links
- More information: https://manned.org/sysctl.8
