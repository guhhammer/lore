# apt-get

Utilitaire de gestion des paquets Debian et Ubuntu. Recherche des paquets en utilisant `apt-cache`. Il est recommandé d’utiliser `apt` pour une utilisation interactive dans les versions 16.04 et ultérieures d’Ubuntu.

## Usage
- `sudo apt-get update` : mise à jour de la liste des paquets et des versions disponibles (il est recommandé de l'exécuter avant les autres commandes `apt-get`)
- `sudo apt-get install package` : installation d'un paquet, ou mise à jour avec la dernière version disponible
- `sudo apt-get remove package` : suppression d'un paquet
- `sudo apt-get purge package` : suppression d'un paquet et de ses fichiers de configuration
- `sudo apt-get upgrade` : mise à jour de tous les paquets installés vers les dernières versions disponibles
- `sudo apt-get autoclean` : nettoyage du dépôt local - supprime les fichiers de paquets (`.deb`) des téléchargements interrompus qui ne peuvent plus être téléchargés
- `sudo apt-get autoremove` : suppression de tous les paquets qui ne sont plus nécessaires
- `sudo apt-get dist-upgrade` : mise à jour des paquets installés (comme la commande `upgrade`), mais avec suppression des paquets obsolètes et installation des paquets supplémentaires pour répondre aux nouvelles dépendances

## Links
- More information: https://manned.org/apt-get.8
