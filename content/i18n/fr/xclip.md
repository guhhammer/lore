# xclip

Outil de manipulation de presse-papiers X11, semblable à `xsel`. Gère les sélections primaires et secondaires de X, en plus du presse-papier système (`<Ctrl c>`/`<Ctrl v>`).

## Usage
- `echo 123 | xclip` : copie la sortie d'une commande vers la zone de sélection primaire de X11 (presse-papiers)
- `echo 123 | xclip -selection primary|secondary|clipboard` : copie la sortie d'une commande vers une zone de sélection de X11 donnée
- `echo 123 | xclip -sel clip` : copie le contenu d'un fichier vers le presse-papiers système, avec la notation courte
- `xclip -sel clip fichier_entrée.txt` : copie le contenu d'un fichier vers le presse-papiers système
- `xclip -sel clip -t image/png fichier_entrée.png` : copie le contenu d'une image PNG vers le presse-papiers système (peut être collé dans d'autres programmes correctement)
- `xclip -o` : colle le contenu de la zone de sélection de X11 à la console
- `xclip -o -sel clip` : colle le contenu du presse-papier système à la console

## Links
- More information: https://manned.org/xclip
