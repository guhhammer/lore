# osascript

Ejecuta AppleScript o JavaScript for Automation (JXA) desde la línea de comandos.

## Usage
- `osascript -e 'say "Hello world"'` : ejecuta un comando AppleScript
- `osascript -e 'say "Hello"' -e 'say "world"'` : ejecuta varios comandos AppleScript
- `osascript ruta/al/apple.scpt` : ejecuta un archivo AppleScript compilado (`*.scpt`), empaquetado (`*.scptd`) o un archivo Applescript en texto plano (`*.applescript`)
- `osascript -e 'id of app "Application"'` : obtén el identificador del paquete de una aplicación (útil para `open -b`)
- `osascript -l JavaScript -e "console.log('Hola mundo');"` : ejecuta un comando JavaScript
- `osascript -l JavaScript ruta/al/script.js` : ejecuta un archivo JavaScript

## Links
- More information: https://keith.github.io/xcode-man-pages/osascript.1.html
