# rename

Hernoem een bestand of groep van bestanden met een `regex`. WAARSCHUWING: Dit commando overschrijft bestanden zonder bevestiging, tenzij de `-n` optie gebruikt wordt. Opmerking: Deze pagina verwijst naar de Perl-versie, ook bekend als `file-rename`.

## Usage
- `rename 's/van/naar/' *.txt` : vervang `van` met `naar` in de bestandsnamen van de opgegeven bestanden
- `rename -n 's/van/naar/' *.txt` : toon welke veranderingen zouden plaatsvinden zonder ze uit te voeren (dry-run)
- `rename 's/\.oud$/\.nieuw/' *.txt` : verander de extensie
- `rename -f 'y/A-Z/a-z/' *.txt` : verander naar kleine letters (gebruik `-f` in hoofdletterongevoelige bestandssystemen)
- `rename -f 's/\b(\w)/\U$1/g' *.txt` : schrijf de eerste letter van elk woord in de naam met een hoofdletter
- `rename 's/\s+/_/g' *.txt` : vervang spaties met underscores

## Links
- More information: https://manned.org/prename
