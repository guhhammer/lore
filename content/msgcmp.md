# msgcmp

Compare message catalog and template.

## Usage
- `msgcmp [OPTION] def.po ref.pot` : the general form, as the manual gives it
- `msgcmp -D` : add DIRECTORY to list for input files search
- `msgcmp -m` : apply ref.pot to each of the domains in def.po
- `msgcmp -N` : do not use fuzzy matching
- `msgcmp --use-fuzzy` : consider fuzzy entries

## Links
- Documentation: https://savannah.gnu.org/projects/gettext
