# daemonize

Lance une commande (qui ne se "démonise" pas elle-même) comme démon UNIX.

## Usage
- `daemonize commande arguments_commande` : lance une commande comme démon
- `daemonize -p chemin/vers/fichier/pid commande arguments_commande` : écrit le PID dans le fichier spécifié
- `daemonize -l chemin/vers/fichier/verrou commande arguments_commande` : utilise un fichier verrou pour s'assurer que seulement une instance fonctionne à la fois
- `sudo daemonize -u utilisateur commande arguments_commande` : utilise le compte utilisateur spécifié

## Links
- More information: https://software.clapper.org/daemonize/
