# ccache

Caché del compilador C/C++. Nota: los paquetes suelen proporcionar enlaces simbólicos para los compiladores en `/usr/lib/ccache/bin`. Anteponga este directorio en `$PATH` para utilizar automáticamente `ccache` con los compiladores.

## Usage
- `ccache -s` : muestra las estadísticas de la caché actual
- `ccache -C` : borra toda la caché
- `ccache -z` : restablece (zero) las estadísticas (pero no la propia caché)
- `ccache gcc ruta/al/archivo.c` : compila código C y almacena la salida compilada en la caché (para usar `ccache` en todas las invocaciones de `gcc`, lea la nota anterior)

## Links
- More information: https://ccache.dev/manual/latest.html
