# logcat

Vuelca un registro de mensajes del sistema, incluyendo seguimientos de pila cuando ocurren errores, y mensajes informativos enviados por las aplicaciones.

## Usage
- `logcat` : muestra registros del sistema
- `logcat -f ruta/al/archivo` : escribe registros del sistema a un archivo
- `logcat --regex expresión_regular` : muestra registros que coincidan con una `regex`
- `logcat --pid pid` : muestra registros de un proceso específico
- `logcat --pid $(pidof -s paquete)` : muestra registros del proceso de un paquete específico

## Links
- More information: https://developer.android.com/tools/logcat
