# sudo

Exécute une commande unique en tant que super-utilisateur (super-user) ou un autre utilisateur.

## Usage
- `sudo less /var/log/syslog` : exécute une commande en tant que super-utilisateur
- `sudo -e /etc/fstab` : édite un fichier en tant que super-utilisateur avec votre éditeur par défaut
- `sudo -u utilisateur -g groupe id -a` : exécute une commande en tant qu'un autre utilisateur et/ou groupe
- `sudo !!` : répète la dernière commande préfixée de `sudo` (uniquement dans Bash, Zsh, etc.)
- `sudo -i` : lance le terminal par défaut avec des privilèges de super-utilisateur et exécute des fichiers à profil spécifique (`.profile`, `.bash_profile`, etc.)
- `sudo -s` : lance le terminal par défaut avec des privilèges de super-utilisateur sans modifier l'environnement
- `sudo -i -u utilisateur` : lance le terminal par défaut en tant que l'utilisateur spécifié, en chargeant l'environnement de cet utilisateur et en lisant les fichiers à profil spécifique de cet utilisateur (`.profile`, `.bash_profile`, etc.)
- `sudo -ll` : liste les commandes autorisées (et interdites) pour l'utilisateur actuel

## Links
- More information: https://www.sudo.ws/sudo.html
