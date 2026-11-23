# alien

Convertit différents paquets d'installation vers d'autres formats.

## Usage
- `sudo alien -d chemin/vers/fichier` : convertit un fichier d'installation spécifique vers le format Debian (extension `.deb`)
- `sudo alien -r chemin/vers/fichier` : convertit un fichier d'installation spécifique vers le format Red Hat (extension `.rpm`)
- `sudo alien -t chemin/vers/fichier` : convertit un fichier d'installation spécifique en un fichier d'installation Slackware (extension `.tgz`)
- `sudo alien -d -i chemin/vers/fichier` : convertit un fichier d'installation spécifique vers le format Debian et l'installe sur le système

## Links
- More information: https://manned.org/alien
