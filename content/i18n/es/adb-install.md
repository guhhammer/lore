# adb install

Instalación de Android Debug Bridge: Envía paquetes a una instancia del emulador de Android o a dispositivos Android conectados.

## Usage
- `adb install ruta/al/archivo.apk` : envía una aplicación Android a un emulador/dispositivo
- `adb -s numero_de_serie install ruta/al/archivo.apk` : envía una aplicación Android a un emulador/dispositivo específico (ignora `$ANDROID_SERIAL`)
- `adb install -r ruta/al/archivo.apk` : reinstala una aplicación existente, manteniendo sus datos
- `adb install -d ruta/al/archivo.apk` : envía una aplicación Android permitiendo bajar el código de versión (sólo paquetes depurables)
- `adb install -g ruta/al/archivo.apk` : concede todos los permisos enumerados en el manifiesto de la aplicación
- `adb install --fastdeploy ruta/al/archivo.apk` : actualiza rápidamente un paquete instalado actualizando sólo las partes del APK que han cambiado

## Links
- More information: https://developer.android.com/tools/adb
