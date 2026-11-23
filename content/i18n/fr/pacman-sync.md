# pacman-sync

Synchronise les paquets depuis les dépôts distants.

## Usage
- `sudo pacman -S paquet` : installe un nouveau paquet
- `sudo pacman -Syu` : synchronise et actualise la base de données des paquets en effectuant aussi une mise à niveau du système (ajouter `--downloadonly` pour télécharger uniquement les paquets sans les mettre à jour) :`
- `sudo pacman -Syu --noconfirm paquet` : synchronise, met à jour et installe un paquet sans demander de confirmation
- `pacman -Ss "motif_recherche"` : recherche un paquet en utilisant un mot-clé ou une `regex`
- `pacman -Si paquet` : affiche des informations sur un paquet
- `sudo pacman -Syu --overwrite chemin/vers/fichier` : ecrit par-dessus des fichiers en conflit pendant une mise à jour
- `sudo pacman -Sc` : supprime les fichiers concernant des paquets non installés et les dépôts inutilisés du cache de pacman (utiliser les options `Scc` pour nettoyer tous les paquets)
- `sudo pacman -S paquet=version` : spécifie la version du paquet à installer

## Links
- More information: https://manned.org/pacman.8
