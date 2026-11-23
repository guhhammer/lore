# abbr

Gestiona abreviatures per la shell fish. Les paraules definides per l'usuari es reemplacen per expresions llarges en introduïr-les.

## Usage
- `abbr -a nom_abreviatura comandament arguments` : afegeix una nova abreviatura
- `abbr --rename antic_nom nou_nom` : canvia el nom d'una abreviatura existent
- `abbr -e nom_abreviatura` : esborra una abreviatura existent
- `ssh nom_host abbr -s | source` : importa les abreviatures definides en un altre host per SSH

## Links
- More information: https://fishshell.com/docs/current/cmds/abbr.html
