# clear

Efface l'écran du terminal.

## Usage
- `clear` : efface l'écran (identique à la séquence `<Ctrl l>` sur une interface Bash)
- `clear -x` : efface l'écran mais conserve le tampon de défilement du terminal
- `clear -T type_de_terminal` : indique le type de terminal à effacer (utilise par défaut la variable d'environnement `$TERM`)
- `clear -V` : affiche la version de `ncurses` utilisée par `clear`

## Links
- More information: https://manned.org/clear
