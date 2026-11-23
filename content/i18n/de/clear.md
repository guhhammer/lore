# clear

Leert den Bildschirm eines Terminals.

## Usage
- `clear` : leere den Bildschirm (äquivalent zu `<Ctrl l>` in einer Bash Shell)
- `clear -x` : leere den Bildschirm, aber erhalte den Rückscroll-Puffer des Terminals
- `clear -T typ_des_terminals` : lege den Typ des zu leerenden Terminals fest (Standardwert ist die Umgebungsvariable `$TERM`)
- `clear -V` : zeige die Version von `ncurses` an, die von `clear` benutzt wird

## Links
- More information: https://manned.org/clear
