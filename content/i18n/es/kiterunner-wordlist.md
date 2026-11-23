# kiterunner wordlist

Un escáner web contextual para gestionar las listas de palabras utilizadas en la detección de API y puntos finales web. El subcomando `wordlist` se encarga de mostrar y guardar las listas de palabras en `~/.cache/kiterunner`.

## Usage
- `kiterunner wordlist list` : muestra todas las listas de palabras de Assetnote almacenadas en caché y disponibles
- `kiterunner wordlist list -o json` : muestra las listas de palabras con salida en formato JSON
- `kiterunner wordlist list -v debug` : muestra las listas de palabras con salida de depuración detallada
- `kiterunner wordlist save apiroutes-210328` : guarda una lista de palabras específicas de Assetnote mediante un alias
- `kiterunner wordlist save ruta/a/httparchive_apiroutes_2024_05_28.txt` : guarda una lista de palabras específicas de Assetnote por el nombre completo del archivo
- `kiterunner wordlist save apiroutes-210328,aspx-210328` : guarda varias listas de palabras mediante un alias
- `kiterunner wordlist save apiroutes-210328 -q` : guarda una lista de palabras en modo silencioso para suprimir la salida

## Links
- More information: https://github.com/assetnote/kiterunner#usage
