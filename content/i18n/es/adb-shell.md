# adb shell

Android Debug Bridge Shell: Ejecuta comandos shell remotos en una instancia del emulador de Android o en dispositivos Android conectados.

## Usage
- `adb shell` : inicia una shell interactiva remota en el emulador o dispositivo
- `adb shell getprop` : obtén todas las propiedades del emulador o dispositivo
- `adb shell pm reset-permissions` : revierte todos los permisos de ejecución a sus valores por defecto
- `adb shell pm revoke paquete permission` : revoca un permiso peligroso para una aplicación
- `adb shell input keyevent keycode` : activa un evento de clave
- `adb shell pm clear paquete` : borra los datos de una aplicación en un emulador o dispositivo
- `adb shell am start -n paquete/activity` : inicia una actividad en el emulador o dispositivo
- `adb shell am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : inicia la actividad de inicio en un emulador o dispositivo

## Links
- More information: https://developer.android.com/tools/adb
