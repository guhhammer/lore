# hexyl

Un simple visualiseur hexadécimal pour le terminal. Il utilise des couleurs pour distinguer les différentes catégories d'octets.

## Usage
- `hexyl chemin/vers/fichier` : affiche la représentation hexadécimale d'un fichier
- `hexyl -n n chemin/vers/fichier` : affiche la représentation hexadécimale des n premiers octets d'un fichier
- `hexyl -r 512:1024 chemin/vers/fichier` : affiche les octets 512 à 1024 d'un fichier
- `hexyl -r 1024:+512 chemin/vers/fichier` : affiche 512 octets en commençant par le 1024e octet

## Links
- More information: https://github.com/sharkdp/hexyl/blob/master/doc/hexyl.1.md
