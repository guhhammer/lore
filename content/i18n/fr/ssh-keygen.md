# ssh-keygen

Génère des clés SSH. Utilisées entre autres pour l'authentification ou la connexion sans utiliser de mot de passe.

## Usage
- `ssh-keygen` : génère une clé de manière interactive
- `ssh-keygen -t ed25519 -a 32 -f ~/.ssh/fichier` : génère une clé ed25519, avec 32 passages de fonction de dérivation de clé et enrigster dans un fichier spécifique
- `ssh-keygen -t rsa -b 4096 -C "commentaire|email"` : génère une clé RSA de 4096 bits, avec l'adresse électronique en commentaire
- `ssh-keygen -R hote_distant` : retire les clés d'une machine donnée du fichier `known_hosts` des hôtes connus (utile lorsque un hôte déjà enregistré change de clé)
- `ssh-keygen -l -E md5 -f ~/.ssh/fichier` : affiche l'empreinte d'une clé sous format d'un hash MD5
- `ssh-keygen -p -f ~/.ssh/fichier` : change le mot de passe d'une clé
- `ssh-keygen -p -N "" -m PEM -f ~/.ssh/cle_privee_OpenSSH` : change le format d'une clé (par exemple du format OPENSSH vers PEM), le fichier étant réécrit

## Links
- More information: https://man.openbsd.org/ssh-keygen
