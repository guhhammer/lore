# rubocop

Analiza archivos de Ruby.

## Usage
- `rubocop` : verifica todos los archivos en el directorio actual (incluyendo subdirectorios)
- `rubocop ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : verifica uno o más archivos o directorios determinados
- `rubocop --out ruta/al/archivo` : guarda la salida en un archivo
- `rubocop --show-cops` : muestra la lista de cops (reglas de análisis)
- `rubocop --except cop1 cop2 ...` : excluye una regla
- `rubocop --only cop1 cop2 ...` : ejecuta solo determinadas reglas
- `rubocop --auto-correct` : autocorrige archivos (experimental)

## Links
- More information: https://docs.rubocop.org/rubocop/latest/usage/cli_reference.html
