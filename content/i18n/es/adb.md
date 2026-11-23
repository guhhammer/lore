# adb

Android Debug Bridge: comunica con una instancia del emulador Android o con dispositivos Android conectados. Algunos subcomandos como `shell` tienen su propia documentación de uso.

## Usage
- `adb start-server` : comprueba si el proceso del servidor adb se está ejecutando y lo inicia
- `adb kill-server` : finaliza el proceso del servidor adb
- `adb shell` : inicia una consola remota en el emulador/dispositivo de destino
- `adb install -r ruta/al/archivo.apk` : instala una aplicación Android en un emulador/dispositivo
- `adb pull ruta/al/archivo_o_directorio_del_dispositivo ruta/al/directorio_de_destino_local` : copia un archivo/directorio del dispositivo de destino
- `adb push ruta/al/archivo_o_directorio_local ruta/al/dirección_o_destino_del_dispositivo` : copia un archivo/directorio al dispositivo de destino
- `adb devices` : lista todos los dispositivos conectados
- `adb -s id_del_dispositivo shell` : especifica a qué dispositivo envía los comandos si hay varios dispositivos

## Links
- More information: https://developer.android.com/tools/adb
