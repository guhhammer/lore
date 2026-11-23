# spctl

Gestiona el subsistema de políticas de evaluación de seguridad. Utilidad para gestionar Gatekeeper en macOS.

## Usage
- `spctl --master-disable` : desactiva Gatekeeper
- `spctl --add --label nombre_regla ruta/al/archivo` : añade una regla para permitir la ejecución de una aplicación (el etiquetado de la regla es opcional)
- `spctl --master-enable` : activa Gatekeeper
- `spctl --list` : lista todas las reglas del sistema

## Links
- More information: https://keith.github.io/xcode-man-pages/spctl.8.html
