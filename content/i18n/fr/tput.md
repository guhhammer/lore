# tput

Accède et modifie les paramètres du terminal.

## Usage
- `tput cup coordonnée_y coordonnée_x` : déplace le curseur à un endroit donné sur l'écran
- `tput setaf|setab code_de_couleur_ANSI` : règle la couleur de l'avant-plan (af) ou de l'arrière-plan (ab)
- `tput cols|lines|colors` : affiche le numéro de colonnes, de rangées, ou de couleurs
- `tput bel` : active la sonnerie du terminal
- `tput sgr0` : réinitialise tous les attributs du terminal
- `tput smam|rmam` : active ou désactive le retour automatique à la ligne

## Links
- More information: https://manned.org/tput
