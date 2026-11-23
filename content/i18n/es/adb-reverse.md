# adb reverse

Android Debug Bridge Reverse: conexiones de socket inversas desde una instancia de emulador de Android o dispositivos Android conectados.

## Usage
- `adb reverse --list` : lista todas las conexiones de socket inverso de emuladores y dispositivos
- `adb reverse tcp:remote_port tcp:local_port` : invierte un puerto TCP desde un emulador o dispositivo a localhost
- `adb reverse --remove tcp:remote_port` : elimina una conexión de socket inversa de un emulador o dispositivo
- `adb reverse --remove-all` : elimina todas las conexiones de socket inverso de todos los emuladores y dispositivos

## Links
- More information: https://developer.android.com/tools/adb
