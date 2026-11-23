# kiterunner kb

Un escáner web contextual para manipular esquemas de KiteBuilder utilizados en el descubrimiento de API y puntos finales web. El subcomando `kb` se encarga de la compilación, conversión, análisis y reproducción de solicitudes de esquemas.

## Usage
- `kiterunner kb compile ruta/a/lista_de_palabras.json ruta/a/lista_de_palabras.kite` : compila un esquema de KiteBuilder desde JSON a un archivo Kite
- `kiterunner kb convert ruta/a/lista_de_palabras.kite ruta/a/lista_de_palabras.txt` : convierte un archivo Kite en una lista de palabras de texto
- `kiterunner kb convert ruta/a/lista_de_palabras.txt ruta/a/lista_de_palabras.kite` : convierte una lista de palabras en formato de texto a un archivo Kite
- `kiterunner kb convert ruta/a/lista_de_palabras.kite ruta/a/lista_de_palabras.json` : convierte un archivo Kite a un esquema JSON
- `kiterunner kb parse ruta/a/lista_de_palabras.json -o json` : analiza un esquema de KiteBuilder y generar datos JSON formateados
- `kiterunner kb parse ruta/a/lista_de_palabras.kite -o texto` : analiza un archivo Kite y genera datos de texto formateados
- `kiterunner kb replay -w ruta/a/lista_de_palabras.kite "solicitud_salida"` : reproduce una solicitud específica a partir de la salida de un esquema de KiteBuilder
- `kiterunner kb replay -w ruta/a/lista_de_palabras.kite -p http://localhost:8080 "solicitud_salida"` : reproduce una solicitud a través de un proxy para su inspección

## Links
- More information: https://github.com/assetnote/kiterunner#usage
