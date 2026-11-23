# kiterunner brute

Un escáner web contextual para realizar ataques de fuerza bruta en rutas de API y puntos finales web utilizando listas de palabras. El subcomando `brute` se dirige a uno o varios hosts.

## Usage
- `kiterunner brute https://example.com -A apiroutes-210328:20000` : realiza un ataque de fuerza bruta a un objetivo con una lista de palabras de Assetnote (por ejemplo, las primeras 20 000 rutas de API)
- `kiterunner brute https://example.com -w ruta/a/lista_palabras.txt` : realiza un ataque de fuerza bruta a un objetivo con una lista de palabras personalizada
- `kiterunner brute https://example.com -w ruta/a/dirsearch.txt -D -e json,txt` : realiza un ataque de fuerza bruta utilizando una lista de palabras al estilo dirsearch con sustitución de extensiones
- `kiterunner brute https://example.com -w ruta/a/lista_palabras.txt -e aspx,ashx -o json` : ataque de fuerza bruta con extensiones de archivo específicas añadidas y salida en formato JSON
- `kiterunner brute ruta/a/objetivos.txt -w ruta/a/lista_palabras.txt -x 5 -j 100` : ataque de fuerza bruta a una lista de objetivos desde un archivo con ajustes de concurrencia personalizados para mejorar el rendimiento
- `kiterunner brute https://example.com -w ruta/a/lista_palabras.txt --ignore-length 100-105` : realiza un ataque de fuerza bruta e ignora respuestas con una longitud de contenido específica
- `kiterunner brute https://example.com -w ruta/a/lista_palabras.txt -H "Authorization: Bearer token"` : ataque de fuerza bruta con encabezados HTTP personalizados
- `kiterunner brute ruta/a/objetivos.txt -w ruta/a/lista_palabras.txt --fail-status-codes 400,401,404` : ataque de fuerza bruta a una lista de objetivos desde un archivo con filtrado de códigos de estado de error

## Links
- More information: https://github.com/assetnote/kiterunner#usage
