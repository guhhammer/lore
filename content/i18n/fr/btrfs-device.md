# btrfs device

Gestion des partitions dans un système de fichiers BTRFS.

## Usage
- `sudo btrfs d a chemin/vers/block_device1 chemin/vers/block_device2 ... chemin/vers/systeme_de_fichiers_btrfs` : ajoute un ou plusieurs périphériques à un système de fichiers btrfs
- `sudo btrfs d rem chemin/vers/peripherique1|identifiant_peripherique1 chemin/vers/peripherique2|identifiant_peripherique2 ...` : retire un périphérique d'un système de fichiers btrfs
- `sudo btrfs d st chemin/vers/systeme_de_fichiers_btrfs` : affiche les statistiques d'erreurs
- `sudo btrfs d sc -d` : scanne tous les disques et informer le noyau de tous les sytèmes de fichiers btrfs détectés
- `sudo btrfs d u chemin/vers/systeme_de_fichiers_btrfs` : affiche les statistiques détaillées d'allocation par disque

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-device.html
