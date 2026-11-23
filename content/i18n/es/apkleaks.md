# apkleaks

Expone URIs, endpoints y secretos de archivos APK. Nota: APKLeaks utiliza el desensamblador `jadx` para decompilar archivos APK.

## Usage
- `apkleaks -f ruta/al/archivo.apk` : escanea un archivo APK en busca de URIs, endpoints y secretos
- `apkleaks -f ruta/al/archivo.apk -o ruta/al/archivo.txt` : escanea y guarda el resultado en un archivo específico
- `apkleaks -f ruta/al/archivo.apk -a "--threads-count 5 --deobf"` : pasa argumentos del desensamblador `jadx`

## Links
- More information: https://github.com/dwisiswant0/apkleaks
