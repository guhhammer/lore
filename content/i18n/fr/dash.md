# dash

Debian Almquist SHell, une implémentation de `sh` moderne, conforme à POSIX (non compatible avec Bash).

## Usage
- `dash` : démarre une session shell interactive
- `dash -c "commande"` : exécute une commande, puis termine la session
- `dash chemin/vers/le/script.sh` : exécute un script
- `dash -x chemin/vers/le/script.sh` : exécute un script en affichant chaque commande avant de l'exécuter
- `dash -e chemin/vers/le/script.sh` : exécute un script en s'arrêtant à la première erreur
- `dash -s` : lit et exécute des commandes depuis l'entrée standard `stdin`

## Links
- More information: https://manned.org/dash
