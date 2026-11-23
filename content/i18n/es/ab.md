# ab

Herramienta comparativa (benchmark) del servidor Apache HTTP.

## Usage
- `ab -n 100 url` : ejecuta 100 solicitudes HTTP GET a una URL dada
- `ab -n 100 -c 10 url` : ejecuta 100 solicitudes HTTP GET, en lotes simultáneos de a 10, a una URL
- `ab -n 100 -T application/json -p ruta/al/archivo.json url` : ejecuta 100 solicitudes HTTP POST a una URL, utilizando la carga JSON de un archivo
- `ab -k url` : utiliza HTTP Keep Alive, es decir, realiza múltiples solicitudes dentro de una sesión HTTP
- `ab -t 60 url` : establece el máximo número de segundos utilizados para la comparación
- `ab -e ruta/al/archivo.csv` : escribe los resultados a un archivo CSV

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
