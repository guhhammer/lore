# xsel

Outil de sélection et de manipulation du presse-papiers X11.

## Usage
- `echo 123 | xsel -ib` : utilise la sortie d'une commande comme entrée du presse-papiers (équivalent de `<Ctrl c>`)
- `cat chemin/vers/fichier | xsel -ib` : utilise le contenu d'un fichier comme entrée du presse-papiers
- `xsel -ob` : affiche le contenu du presse-papiers dans le terminal (équivalent à `<Ctrl v>`)
- `xsel -ob >chemin/vers/fichier` : sortie du contenu du presse-papiers dans un fichier
- `xsel -cb` : efface le presse-papiers
- `xsel -op` : affiche le contenu de la sélection primaire X11 dans le terminal (équivalent à `<MiddleClick>`)

## Links
- More information: https://manned.org/xsel
