# adb forward

Permite conectarse a un dispositivo Android de forma inalámbrica.

## Usage
- `adb forward tcp:puerto_local tcp:puerto_remoto` : reenvía un puerto TCP
- `adb forward --list` : enumera todos los reenvíos
- `adb forward --remove tcp:puerto_local` : elimina una regla de reenvío
- `adb forward --remove-all` : elimina todas las reglas de reenvío

## Links
- More information: https://developer.android.com/tools/adb
