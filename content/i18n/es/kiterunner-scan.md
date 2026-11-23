# kiterunner scan

Un escáner web contextual para escanear simultáneamente rutas API y puntos finales web utilizando listas de palabras de kitebuilder. El subcomando `scan` se dirige a uno o varios hosts con solicitudes API estructuradas.

## Usage
- `kiterunner scan https://example.com -A apiroutes-210228:5000` : escanea un objetivo con una lista de palabras de Assetnote (por ejemplo, las primeras 5000 rutas de la API)
- `kiterunner scan https://example.com -w ruta/a/lista_de_palabras.kite` : analiza un objetivo con una lista de palabras de Kitebuilder
- `kiterunner scan ruta/a/hosts.txt -w ruta/a/lista_de_palabras.kite` : analiza varios hosts desde un archivo con una lista de palabras de Kitebuilder
- `kiterunner scan https://example.com -A apiroutes-210228:5000 -o json` : realiza un escaneo con una lista de palabras de Assetnote y salida en formato JSON
- `kiterunner scan https://example.com -w ruta/a/lista_de_palabras.kite -x 5 -j 100` : realiza un escaneo con ajustes de concurrencia personalizados para mejorar el rendimiento
- `kiterunner scan https://example.com -w ruta/a/rafter.txt -d 0` : realiza un escaneo con una lista de palabras como si fuera una lista de palabras normal, desactivando el escaneo en profundidad
- `kiterunner scan https://example.com -w ruta/a/lista_de_palabras.kite -H "Autorización: Token de portador" --ignore-length 100-105` : realiza un escaneo con encabezados personalizados e ignora respuestas con una longitud de contenido específica
- `kiterunner scan https://example.com -w ruta/a/lista_de_palabras.kite --kitebuilder-full-scan` : realiza un escaneo completo con Kitebuilder sin escaneo por fases

## Links
- More information: https://github.com/assetnote/kiterunner#usage
