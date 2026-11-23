# cut

Corta campos de `stdin` o archivos.

## Usage
- `comando | cut --characters|field 1|1,10|1-10|1-|-10` : imprime un rango específico de caracteres/campos de cada línea
- `comando | cut --delimiter "," --fields 1` : imprime un rango de campos de cada línea con un delimitador específico
- `cut --characters 1 ruta/al/archivo` : imprime un rango de caracteres de cada línea del archivo específico
- `comando | cut --zero-terminated --fields 1` : imprime campos específicos de líneas terminadas en `NUL` (por ejemplo, como en `find . -print0`) en lugar de nuevas líneas

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cut-invocation.html
