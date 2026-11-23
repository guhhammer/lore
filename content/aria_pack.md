# aria_pack

Generate compressed, read-only Aria tables.

## Usage
- `aria_pack [OPTIONS] filename...` : the general form, as the manual gives it
- `aria_pack -b` : make a backup of the table as table_name.OLD
- `aria_pack --character-sets-dir=name` : directory where character sets are
- `aria_pack -f` : force packing of table even if it gets bigger or if tempfile exists
- `aria_pack -s` : be more silent

## Links
- Documentation: https://mariadb.com/docs/
