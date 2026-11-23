# ed

El editor de texto original de Unix.

## Usage
- `ed` : inicia una sesión de edición interactiva con un documento vacío
- `ed -p '> '` : inicia una sesión de edición interactiva con un documento vacío y un prompt específico
- `ed -v` : inicia una sesión de edición interactiva con errores de usuario
- `ed -q -s` : inicia una sesión de edición interactiva con un documento vacío y sin diagnósticos, recuento de bytes ni indicaciones '!'
- `ed -l` : inicia una sesión de edición interactiva sin cambio de estado de salida cuando falla el comando
- `ed ruta/al/archivo` : edita un archivo específico (muestra el recuento de bytes del archivo cargado)
- `,s/regex/replacement/g` : [Interactivo] Reemplaza una cadena con un reemplazo específico para todas las líneas
- `q` : [Interactivo] Sale de `ed`

## Links
- More information: https://www.gnu.org/software/ed/manual/ed_manual.html
