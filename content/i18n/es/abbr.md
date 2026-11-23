# abbr

Administra abreviaturas del intérprete de comandos fish. Las palabras definidas por el usuario se reemplazan con frases más largas después de ingresarlas.

## Usage
- `abbr -a nombre_abreviatura comando argumentos_del_comando` : añade una nueva abreviatura
- `abbr --rename nombre_antiguo nombre_nuevo` : cambia el nombre de una abreviatura existente
- `abbr -e nombre_abreviatura` : borra una abreviatura existente
- `ssh nombre_host abbr -s | source` : importa las abreviaturas definidas en otro host a través de SSH

## Links
- More information: https://fishshell.com/docs/current/cmds/abbr.html
