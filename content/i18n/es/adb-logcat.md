# adb logcat

Vuelca un registro de mensajes del sistema.

## Usage
- `adb logcat` : muestra registros del sistema
- `adb logcat -e expresión_regular` : muestra las líneas que coincidan con una `regex`
- `adb logcat etiqueta:modo *:S` : muestra los registros de una etiqueta en un modo específico (Verboso, Depuración, Información, Warning, Error, Fatal, Silencioso), filtrando otras etiquetas
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : muestra los registros de aplicaciones React Native en modo Verboso Silenciando otras etiquetas
- `adb logcat *:W` : muestra los registros de todas las etiquetas con nivel de prioridad Warning y superior
- `adb logcat --pid pid` : muestra los registros de un proceso específico
- `adb logcat --pid $(adb shell pidof -s paquete)` : muestra los registros del proceso de un paquete específico
- `adb logcat -v color` : colorea el registro (normalmente se usan filtros)

## Links
- More information: https://developer.android.com/tools/logcat
