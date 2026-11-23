# caffeinate

Evita que macOS se duerma.

## Usage
- `caffeinate -u -t 3600` : evita que macOS entre en reposo durante 1 hora (3600 segundos)
- `caffeinate -s "comando"` : evita que entre en reposo hasta que termine de ejecutarse un comando
- `caffeinate -w pid` : evita que el sistema entre en reposo hasta que finalice un proceso con el PID especificado
- `caffeinate -i` : evita que entre en reposo (usa `<Ctrl c>` para salir)
- `caffeinate -m` : evita que el disco entre en reposo (usa `<Ctrl c>` para salir)

## Links
- More information: https://keith.github.io/xcode-man-pages/caffeinate.8.html
