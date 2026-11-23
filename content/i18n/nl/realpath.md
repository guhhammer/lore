# realpath

Toon het opgeloste absolute pad voor een bestand of map.

## Usage
- `realpath pad/naar/bestand_of_map` : toon het absolute pad voor een bestand of map
- `realpath -e pad/naar/bestand_of_map` : vereis dat alle padcomponenten bestaan
- `realpath -L pad/naar/bestand_of_map` : los `..` componenten op voordat symlinks worden gevolgd
- `realpath -s pad/naar/bestand_of_map` : schakel symlink-uitbreiding uit
- `realpath -q pad/naar/bestand_of_map` : onderdruk foutmeldingen
