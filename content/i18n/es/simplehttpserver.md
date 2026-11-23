# simplehttpserver

Un simple servidor HTTP/S que soporta subida de ficheros, autenticación básica y reglas YAML para respuestas personalizadas. Una alternativa Go a `http.server` de Python.

## Usage
- `simplehttpserver -verbose` : inicia el servidor HTTP que sirve el directorio actual con salida verbosa (escucha en todas las interfaces y puerto 8000 por defecto)
- `sudo simplehttpserver -basic-auth usuario:contraseña -path /var/www/html -listen 0.0.0.0:80` : inicia el servidor HTTP con autenticación básica sirviendo una ruta específica a través del puerto 80 en todas las interfaces
- `sudo simplehttpserver -https -domain *.selfsigned.com -listen 0.0.0.0:443` : inicia el servidor HTTP, habilitando HTTPS utilizando un certificado autofirmado con SAN personalizado en todas las interfaces
- `simplehttpserver -upload -header 'X-Powered-By: Go' -header 'Server: SimpleHTTPServer'` : inicia el servidor HTTP con cabeceras de respuesta personalizadas y capacidad de carga
- `simplehttpserver -rules rules.yaml` : inicia el servidor HTTP con reglas personalizables en YAML (consulte la documentación para DSL)

## Links
- More information: https://github.com/projectdiscovery/simplehttpserver
