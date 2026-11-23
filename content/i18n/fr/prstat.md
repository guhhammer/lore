# prstat

Signale les statistiques de processus actifs.

## Usage
- `prstat` : examine tous les processus et rapporte les statistiques triées par utilisation du processeur
- `prstat -s rss` : examine tous les processus et rapporte les statistiques triées par utilisation de la mémoire
- `prstat -t` : rapporte le résumé de l'utilisation totale pour chaque utilisateur
- `prstat -m` : rapporte les informations comptables du processus de micro-état
- `prstat -c -n 5 -s cpu 1` : imprime une liste des 5 meilleurs processeurs utilisant des processus chaque seconde

## Links
- More information: https://www.unix.com/man-page/sunos/1m/prstat
