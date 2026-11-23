# clear

Leegt het scherm van de terminal.

## Usage
- `clear` : maak het scherm leeg
- `clear -x` : maak het scherm leeg maar behoud de scrollbackbuffer van de terminal (gelijk aan het indrukken van `<Ctrl l>` in de Bash-shell)
- `clear -T type_of_terminal` : geef het type terminal aan dat leeggemaakt moet worden (standaard ingesteld op de waarde van de omgevingsvariabele `$TERM`)
- `clear -V` : toon de versie van `ncurses` die door `clear` wordt gebruikt

## Links
- More information: https://manned.org/clear
